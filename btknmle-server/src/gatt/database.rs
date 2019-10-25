use std::collections::BTreeMap;
use std::ops::RangeInclusive;

use bitflags::bitflags;
use bytes::{Buf as _, BufMut as _, Bytes, BytesMut, IntoBuf as _};
use failure::Fail;

use btknmle_pkt::att::{ErrorCode, Handle, Uuid};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "att err {:?}", _0)]
    AttError(ErrorCode),
    #[fail(display = "?")]
    _E,
}

impl From<ErrorCode> for Error {
    fn from(v: ErrorCode) -> Self {
        Self::AttError(v)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

bitflags! {
    pub struct Permissions: u16 {
        const READABLE = 0x0000_0001;
        const WRITABLE = 0x0000_0010;
        const AUTHORIZATION_REQUIRED = 0x0000_0100;
        const AUTHENTICATION_REQUIRED = 0x0000_1000;
    }
}

impl From<CharacteristicProperties> for Permissions {
    fn from(v: CharacteristicProperties) -> Self {
        let mut result = Self::empty();

        if v.contains(CharacteristicProperties::READ) {
            result |= Permissions::READABLE
        }

        if v.contains(CharacteristicProperties::WRITE)
            || v.contains(CharacteristicProperties::WRITE_WITHOUT_RESPONSE)
            || v.contains(CharacteristicProperties::AUTHENTICATED_SIGNED_WRITE)
        {
            result |= Permissions::WRITABLE
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    Service(Uuid),
    Characteristic {
        properties: CharacteristicProperties,
        value_handle: Handle,
        chr_type: Uuid,
    },
    Value(Bytes),
    UTF8(String),
    CCCD(CCCD),
}

impl AttributeValue {
    fn set(&mut self, val: Bytes) -> Option<()> {
        match self {
            AttributeValue::Service(..) => unimplemented!(),
            AttributeValue::Characteristic { .. } => unimplemented!(),
            AttributeValue::Value(..) => {
                *self = AttributeValue::Value(val);
            }
            AttributeValue::UTF8(..) => {
                let v = String::from_utf8_lossy(&val);
                *self = AttributeValue::UTF8(v.to_string());
            }
            AttributeValue::CCCD(..) => {
                let mut val = val.into_buf();
                let v = CCCD::from_bits(val.get_u16_le()).unwrap(); // FIXME
                *self = AttributeValue::CCCD(v);
            }
        }
        Some(())
    }
}

impl From<AttributeValue> for Bytes {
    fn from(v: AttributeValue) -> Self {
        match v {
            AttributeValue::Service(v) => v.into(),
            AttributeValue::Characteristic {
                properties,
                value_handle,
                chr_type,
            } => {
                let mut buf = BytesMut::new();
                buf.put_u8(properties.bits());
                buf.put_u16_le(value_handle.0);
                buf.put(Bytes::from(chr_type));
                buf.freeze()
            }
            AttributeValue::Value(v) => v,
            AttributeValue::UTF8(v) => Bytes::from(v),
            AttributeValue::CCCD(v) => {
                let mut b = BytesMut::new();
                b.put_u16_le(v.bits());
                b.freeze()
            }
        }
    }
}

impl AttributeValue {
    fn size(&self) -> usize {
        match self {
            Self::Service(Uuid::Uuid16(..)) => 2,
            Self::Service(Uuid::Uuid128(..)) => 16,
            Self::Characteristic {
                chr_type: Uuid::Uuid16(..),
                ..
            } => 1 + 2 + 2,
            Self::Characteristic {
                chr_type: Uuid::Uuid128(..),
                ..
            } => 1 + 2 + 16,
            Self::Value(v) => v.len(),
            Self::UTF8(v) => v.as_bytes().len(),
            Self::CCCD(..) => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct Attribute {
    att_handle: Handle,
    att_type: Uuid,
    att_value: AttributeValue,
    att_perm: Permissions,
}

bitflags! {
    pub struct CharacteristicProperties: u8 {
        const BROADCAST = 0x01;
        const READ = 0x02;
        const WRITE_WITHOUT_RESPONSE = 0x04;
        const WRITE = 0x08;
        const NOTIFY = 0x10;
        const INDICATE = 0x20;
        const AUTHENTICATED_SIGNED_WRITE = 0x40;
        const EXTENDED_PROPERTIES = 0x80;
    }
}

bitflags! {
    pub struct CCCD: u16 {
        const NOTIFICATION = 0x0001;
        const INDICATION = 0x0002;
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    mtu: usize,
    attrs: BTreeMap<Handle, Attribute>,
}

impl Database {
    pub fn builder() -> DatabaseBuilder {
        DatabaseBuilder {
            next: 1,
            attrs: BTreeMap::new(),
        }
    }

    pub fn exchange_mtu(&mut self, client_mtu: u16) -> Result<u16> {
        self.mtu = client_mtu as usize;
        Ok(client_mtu)
    }

    fn read_raw(&self, handle: Handle) -> Result<AttributeValue> {
        let att = match self.attrs.get(&handle) {
            Some(att) => att,
            None => return Err(ErrorCode::AttributeNotFound.into()),
        };
        if !att.att_perm.contains(Permissions::READABLE) {
            return Err(ErrorCode::ReadNotPermitted.into());
        }
        Ok(att.att_value.clone())
    }

    pub fn read(&self, handle: Handle) -> Result<Bytes> {
        let val = self.read_raw(handle)?;
        let val = Bytes::from(val);
        Ok(if val.len() > self.mtu - 1 {
            val.slice_to(self.mtu - 1)
        } else {
            val
        })
    }

    pub fn read_blob(&self, handle: Handle, offset: u16) -> Result<Bytes> {
        let val = self.read_raw(handle)?;
        let val = Bytes::from(val).slice_from(offset as usize); // FIXME
        Ok(if val.len() > self.mtu - 1 {
            val.slice_to(self.mtu - 1)
        } else {
            val
        })
    }

    pub fn write(&mut self, handle: Handle, val: impl Into<Bytes>) -> Option<()> {
        let attr = self.attrs.get_mut(&handle);
        match attr {
            Some(attr) => attr.att_value.set(val.into()),
            None => return None,
        };
        Some(())
    }

    pub fn find_information(&self, begin: Handle, end: Handle) -> Result<Vec<(Handle, Uuid)>> {
        let mut iter = self.attrs.range(begin..=end).map(|(_, v)| v);

        let head = match iter.next() {
            Some(item) => item,
            None => return Err(ErrorCode::AttributeNotFound.into()),
        };
        let head_type = head.att_type.clone();
        let type_size = match head_type {
            Uuid::Uuid16(..) => 2,
            Uuid::Uuid128(..) => 16,
        };
        let mut result = vec![(head.att_handle.clone(), head.att_type.clone())];
        let mut size = 2 + type_size;
        while let Some(item) = iter.next() {
            match (&head_type, &item.att_type) {
                (Uuid::Uuid16(..), Uuid::Uuid16(..)) | (Uuid::Uuid128(..), Uuid::Uuid128(..)) => {}
                _ => break,
            };
            result.push((item.att_handle.clone(), item.att_type.clone()));
            size += 2 + type_size;
            if self.mtu < size + 2 + type_size {
                break;
            }
        }

        Ok(result)
    }

    pub fn read_by_type(
        &self,
        begin: Handle,
        end: Handle,
        uuid: Uuid,
    ) -> Result<Vec<(Handle, AttributeValue)>> {
        let mut iter = self.attrs.range(begin..=end).filter_map(|(_, v)| {
            if v.att_type == uuid {
                Some(v)
            } else {
                None
            }
        });

        let head = match iter.next() {
            Some(item) => item,
            None => return Err(ErrorCode::AttributeNotFound.into()),
        };
        let value_size = head.att_value.size();
        let mut result = vec![(head.att_handle.clone(), head.att_value.clone())];
        let mut size = 2 + value_size;
        while let Some(item) = iter.next() {
            if item.att_value.size() != value_size {
                break;
            }
            result.push((item.att_handle.clone(), item.att_value.clone()));
            size += 2 + value_size;
            if self.mtu < size + 2 + value_size {
                break;
            }
        }

        Ok(result)
    }

    pub fn read_by_group_type(
        &self,
        begin: Handle,
        end: Handle,
        uuid: Uuid,
    ) -> Result<Vec<(RangeInclusive<Handle>, AttributeValue)>> {
        let mut iter = self
            .attrs
            .range(begin..)
            .map(|(_, v)| v)
            .skip_while(|v| v.att_type != uuid);

        let mut group = match iter.next() {
            Some(item) => item,
            None => return Err(ErrorCode::AttributeNotFound.into()),
        };
        let value_size = group.att_value.size();
        let mut last = &group.att_handle;

        let mut result = vec![];
        let mut size = 0;
        while let Some(item) = iter.next() {
            if item.att_handle > end {
                return if result.is_empty() {
                    return Err(ErrorCode::AttributeNotFound.into());
                } else {
                    Ok(result)
                };
            };
            if item.att_type == uuid {
                if value_size != group.att_value.size() {
                    return Ok(result);
                };
                let range = RangeInclusive::new(group.att_handle.clone(), last.clone());
                result.push((range, group.att_value.clone()));
                size += 2 + 2 + value_size;
                if self.mtu < size + 2 + 2 + value_size {
                    return Ok(result);
                }
                group = item;
            }
            last = &item.att_handle;
        }
        if value_size == group.att_value.size() {
            let range = RangeInclusive::new(group.att_handle.clone(), last.clone());
            result.push((range, group.att_value.clone()));
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub struct DatabaseBuilder {
    next: u16,
    attrs: BTreeMap<Handle, Attribute>,
}

impl DatabaseBuilder {
    pub fn build(self) -> Database {
        Database {
            mtu: 23,
            attrs: self.attrs,
        }
    }

    pub fn begin_service(&mut self, service: impl Into<Uuid>) {
        let handle = Handle::from(self.next);
        self.next += 1;
        let service = service.into();
        let att = Attribute {
            att_handle: handle.clone(),
            att_type: Uuid::Uuid16(0x2800),
            att_value: AttributeValue::Service(service),
            att_perm: Permissions::READABLE,
        };
        self.attrs.insert(handle, att);
    }

    pub fn with_characteristic(
        &mut self,
        properties: CharacteristicProperties,
        chr_type: impl Into<Uuid>,
        value: impl Into<Bytes>,
    ) -> Handle {
        // FIXME Extended Properties
        let handle = Handle::from(self.next + 0);
        let value_handle = Handle::from(self.next + 1);
        self.next += 2;

        let chr_type = chr_type.into();
        let value = value.into();

        self.attrs.insert(
            handle.clone(),
            Attribute {
                att_handle: handle.clone(),
                att_type: Uuid::Uuid16(0x2803),
                att_value: AttributeValue::Characteristic {
                    properties: properties.clone(),
                    value_handle: value_handle.clone(),
                    chr_type: chr_type.clone(),
                },
                att_perm: Permissions::READABLE,
            },
        );
        self.attrs.insert(
            value_handle.clone(),
            Attribute {
                att_handle: value_handle.clone(),
                att_type: chr_type,
                att_value: AttributeValue::Value(value),
                att_perm: Permissions::from(properties), // FIXME
            },
        );

        value_handle
    }

    pub fn with_user_description(&mut self, description: String) {
        let handle = Handle::from(self.next);
        self.next += 1;
        let att = Attribute {
            att_handle: handle.clone(),
            att_type: Uuid::Uuid16(0x2901),
            att_value: AttributeValue::UTF8(description),
            att_perm: Permissions::READABLE | Permissions::WRITABLE, // FIXME
        };
        self.attrs.insert(handle, att);
    }

    pub fn with_cccd(&mut self, value: CCCD) {
        let handle = Handle::from(self.next);
        self.next += 1;
        let att = Attribute {
            att_handle: handle.clone(),
            att_type: Uuid::Uuid16(0x2902),
            att_value: AttributeValue::CCCD(value),
            att_perm: Permissions::READABLE | Permissions::WRITABLE, // FIXME
        };
        self.attrs.insert(handle, att);
    }

    pub fn with_descriptor(&mut self, uuid: impl Into<Uuid>, value: impl Into<Bytes>) {
        let handle = Handle::from(self.next);
        self.next += 1;
        let att = Attribute {
            att_handle: handle.clone(),
            att_type: uuid.into(),
            att_value: AttributeValue::Value(value.into()),
            att_perm: Permissions::READABLE | Permissions::WRITABLE, // FIXME
        };
        self.attrs.insert(handle, att);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut builder = Database::builder();
        builder.begin_service(Uuid::Uuid16(0x1800)); // #1
        builder.with_characteristic(
            CharacteristicProperties::READ,
            Uuid::Uuid16(0x2A00),
            "MYDEVICENAME",
        ); // #2,3
        builder.with_cccd(CCCD::empty()); // #4
        builder.begin_service(Uuid::Uuid16(0x1801)); // #5
        builder.begin_service(Uuid::Uuid16(0x180A)); // #6
        builder.begin_service(Uuid::Uuid16(0x180F)); // #7
        builder.begin_service(Uuid::Uuid128(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff)); // #8
        builder.begin_service(Uuid::Uuid16(0x1802)); // #9
        let gatt = builder.build();

        let result = gatt.read_by_group_type(
            Handle::from(0x0001),
            Handle::from(0xffff),
            Uuid::Uuid16(0x2800),
        );
        assert_eq!(
            result.unwrap(),
            vec![
                (
                    Handle::from(0x01)..=Handle::from(0x04),
                    AttributeValue::Service(Uuid::Uuid16(0x1800))
                ),
                (
                    Handle::from(0x05)..=Handle::from(0x05),
                    AttributeValue::Service(Uuid::Uuid16(0x1801))
                ),
                (
                    Handle::from(0x06)..=Handle::from(0x06),
                    AttributeValue::Service(Uuid::Uuid16(0x180A))
                ),
                //(Handle::from(0x07)..=Handle::from(0x07), AttributeValue::Service(Uuid::Uuid16(0x180F))),
            ]
        );

        let result = gatt.read_by_group_type(
            Handle::from(0x0007),
            Handle::from(0xffff),
            Uuid::Uuid16(0x2800),
        );
        assert_eq!(
            result.unwrap(),
            vec![(
                Handle::from(0x07)..=Handle::from(0x07),
                AttributeValue::Service(Uuid::Uuid16(0x180F))
            ),]
        );

        let result = gatt.read_by_group_type(
            Handle::from(0x0008),
            Handle::from(0xffff),
            Uuid::Uuid16(0x2800),
        );
        assert_eq!(
            result.unwrap(),
            vec![(
                Handle::from(0x08)..=Handle::from(0x08),
                AttributeValue::Service(Uuid::Uuid128(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff))
            )]
        );

        let result = gatt.read_by_group_type(
            Handle::from(0x0009),
            Handle::from(0xffff),
            Uuid::Uuid16(0x2800),
        );
        assert_eq!(
            result.unwrap(),
            vec![(
                Handle::from(0x09)..=Handle::from(0x09),
                AttributeValue::Service(Uuid::Uuid16(0x1802))
            )]
        );

        let result = gatt.read_by_group_type(
            Handle::from(0x000A),
            Handle::from(0xffff),
            Uuid::Uuid16(0x2800),
        );
        assert_eq!(result.is_err(), true); // FIXME

        let result = gatt
            .read_by_type(Handle::from(0x01), Handle::from(0x04), Uuid::Uuid16(0x2803))
            .unwrap();
        assert_eq!(
            result,
            vec![(
                Handle::from(0x02),
                AttributeValue::Characteristic {
                    properties: CharacteristicProperties::READ,
                    value_handle: Handle::from(0x0003),
                    chr_type: Uuid::Uuid16(0x2A00),
                }
            ),]
        );

        let result = gatt
            .find_information(Handle::from(0x04), Handle::from(0x04))
            .unwrap();
        assert_eq!(result, vec![(Handle::from(0x04), Uuid::Uuid16(0x2902)),]);

        let result = gatt.read_raw(Handle::from(0x03)).unwrap();
        assert_eq!(result, AttributeValue::Value(Bytes::from("MYDEVICENAME")));
    }
}
