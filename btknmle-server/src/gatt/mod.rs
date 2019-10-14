use std::ops::RangeInclusive;
use std::collections::BTreeMap;

use bytes::{Bytes, BytesMut, Buf as _, BufMut as _, IntoBuf as _};
use bitflags::bitflags;

use btknmle_pkt::att::{Handle, Uuid};

bitflags! {
    pub struct Permissions: u16 {
        const WRITE = 0x0000_0001;
        const AUTHORIZATION = 0x0000_0010;
        const AUTHENTICATION = 0x0000_0100;
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
            AttributeValue::Service(..) => {
                unimplemented!()
            },
            AttributeValue::Characteristic { .. } => {
                unimplemented!()
            },
            AttributeValue::Value(..) => {
                *self = AttributeValue::Value(val);
            },
            AttributeValue::UTF8(..) => {
                let v = String::from_utf8_lossy(&val);
                *self = AttributeValue::UTF8(v.to_string());
            },
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
            AttributeValue::Characteristic { properties, value_handle, chr_type } => {
                let mut buf = BytesMut::new();
                buf.put_u8(properties.bits());
                buf.put_u16_le(value_handle.0);
                buf.put(Bytes::from(chr_type));
                buf.freeze()
            },
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
            Self::Characteristic { chr_type: Uuid::Uuid16(..), .. } => 1 + 2 + 2,
            Self::Characteristic { chr_type: Uuid::Uuid128(..), .. } => 1 + 2 + 16,
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
        const Notification = 0x0001;
        const Indication = 0x0002;
    }
}

#[derive(Debug)]
pub struct Database {
    attrs: BTreeMap<Handle, Attribute>,
}

impl Database {
    pub fn builder() -> DatabaseBuilder {
        DatabaseBuilder {
            next: 1,
            attrs: BTreeMap::new(),
        }
    }

    pub fn read(&self, handle: Handle) -> Option<AttributeValue> {
        self.attrs.get(&handle).map(|v| v.att_value.clone())
    }

    pub fn write(&mut self, handle: Handle, val: impl Into<Bytes>) -> Option<()> {
        let attr = self.attrs.get_mut(&handle);
        match attr {
            Some(attr) => attr.att_value.set(val.into()),
            None => return None,
        };
        Some(())
    }

    pub fn find_information(&self, begin: Handle, end: Handle)
        -> Option<Vec<(Handle, Uuid)>> {

        let mut iter = self.attrs
            .range(begin..=end)
            .map(|(_, v)| v);

        let head = match iter.next() {
            Some(item) => item,
            None => return None,
        };
        let expect_size = head.att_value.size();
        let mut result = vec![(head.att_handle.clone(), head.att_type.clone())];
        while let Some(item) = iter.next() {
            if item.att_value.size() != expect_size {
                break
            }
            result.push((item.att_handle.clone(), head.att_type.clone()));
        }

        Some(result)
    }

    pub fn read_by_type(&self, begin: Handle, end: Handle, uuid: Uuid)
        -> Option<Vec<(Handle, AttributeValue)>> {

        let mut iter = self.attrs
            .range(begin..=end)
            .filter_map(|(_, v)| if v.att_type == uuid { Some(v) } else { None });

        let head = match iter.next() {
            Some(item) => item,
            None => return None,
        };
        let expect_size = head.att_value.size();
        let mut result = vec![(head.att_handle.clone(), head.att_value.clone())];
        while let Some(item) = iter.next() {
            if item.att_value.size() != expect_size {
                break
            }
            result.push((item.att_handle.clone(), item.att_value.clone()));
        }

        Some(result)
    }

    pub fn read_by_group_type(&self, begin: Handle, end: Handle, uuid: Uuid)
        -> Option<Vec<(RangeInclusive<Handle>, AttributeValue)>> {

        let mut iter = self.attrs
            .range(begin..)
            .map(|(_, v)| v)
            .skip_while(|v| v.att_type != uuid);

        let mut group = match iter.next() {
            Some(item) => item,
            None => return None,
        };
        let expect_size = group.att_value.size();
        let mut last = &group.att_handle;

        let mut result = vec![];
        while let Some(item) = iter.next() {
            if item.att_handle > end {
                return if result.is_empty() {
                    None
                } else {
                    Some(result)
                };
            };
            if item.att_type == uuid {
                if expect_size != group.att_value.size() {
                    return Some(result)
                };
                let range = RangeInclusive::new(group.att_handle.clone(), last.clone());
                result.push((range, group.att_value.clone()));
                group = item;
            }
            last = &item.att_handle;
        };
        if expect_size == group.att_value.size() {
            let range = RangeInclusive::new(group.att_handle.clone(), last.clone());
            result.push((range, group.att_value.clone()));
        }

        Some(result)
    }
}

#[derive(Debug)]
pub struct DatabaseBuilder {
    next: u16,
    attrs: BTreeMap<Handle, Attribute>,
}

impl DatabaseBuilder {
    pub fn build(self) -> Database {
        Database { attrs: self.attrs, }
    }

    pub fn begin_service(&mut self, service: impl Into<Uuid>) {
        let handle = Handle::from(self.next);
        self.next += 1;
        let service = service.into();
        let att = Attribute {
            att_handle: handle.clone(),
            att_type: Uuid::Uuid16(0x2800),
            att_value: AttributeValue::Service(service),
            att_perm: Permissions::empty(),
        };
        self.attrs.insert(handle, att);
    }

    pub fn with_characteristic(&mut self,
        properties: CharacteristicProperties,
        chr_type: impl Into<Uuid>,
        value: impl Into<Bytes>) -> Handle {

        // FIXME Extended Properties
        let handle = Handle::from(self.next + 0);
        let value_handle = Handle::from(self.next + 1);
        self.next += 2;

        let chr_type = chr_type.into();
        let value = value.into();

        self.attrs.insert(handle.clone(), Attribute {
            att_handle: handle.clone(),
            att_type: Uuid::Uuid16(0x2803),
            att_value: AttributeValue::Characteristic {
                properties,
                value_handle: value_handle.clone(),
                chr_type: chr_type.clone(),
            },
            att_perm: Permissions::empty(),
        });
        self.attrs.insert(value_handle.clone(), Attribute {
            att_handle: value_handle.clone(),
            att_type: chr_type,
            att_value: AttributeValue::Value(value),
            att_perm: Permissions::empty(), // FIXME
        });

        value_handle
    }

    pub fn with_user_description(&mut self, description: String) {
        let handle = Handle::from(self.next);
        self.next += 1;
        let att = Attribute {
            att_handle: handle.clone(),
            att_type: Uuid::Uuid16(0x2901),
            att_value: AttributeValue::UTF8(description),
            att_perm: Permissions::empty(),
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
            att_perm: Permissions::empty(),
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
        builder.with_characteristic(CharacteristicProperties::empty(), Uuid::Uuid16(0x2A00), "MYDEVICENAME"); // #2,3
        builder.with_cccd(CCCD::empty()); // #4
        builder.begin_service(Uuid::Uuid16(0x1801)); // #5
        builder.begin_service(Uuid::Uuid128(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff)); // #6
        builder.begin_service(Uuid::Uuid16(0x1802)); // #7
        let gatt = builder.build();

        let result = gatt.read_by_group_type(Handle::from(0x0001), Handle::from(0xffff), Uuid::Uuid16(0x2800));
        assert_eq!(result, Some(vec![
                (Handle::from(0x01)..=Handle::from(0x04), AttributeValue::Service(Uuid::Uuid16(0x1800))),
                (Handle::from(0x05)..=Handle::from(0x05), AttributeValue::Service(Uuid::Uuid16(0x1801)))
        ]));

        let result = gatt.read_by_group_type(Handle::from(0x0006), Handle::from(0xffff), Uuid::Uuid16(0x2800));
        assert_eq!(result, Some(vec![
                (Handle::from(0x06)..=Handle::from(0x06),
                    AttributeValue::Service(Uuid::Uuid128(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff)))
        ]));

        let result = gatt.read_by_group_type(Handle::from(0x0007), Handle::from(0xffff), Uuid::Uuid16(0x2800));
        assert_eq!(result, Some(vec![
                (Handle::from(0x07)..=Handle::from(0x07), AttributeValue::Service(Uuid::Uuid16(0x1802)))
        ]));

        let result = gatt.read_by_group_type(Handle::from(0x0008), Handle::from(0xffff), Uuid::Uuid16(0x2800));
        assert_eq!(result, None);

        let result = gatt.read_by_type(Handle::from(0x01), Handle::from(0x04), Uuid::Uuid16(0x2803));
        assert_eq!(result, Some(vec![
                (Handle::from(0x02), AttributeValue::Characteristic{
                    properties: CharacteristicProperties::empty(),
                    value_handle: Handle::from(0x0003),
                    chr_type: Uuid::Uuid16(0x2A00),
                }),
        ]));

        let result = gatt.find_information(Handle::from(0x04), Handle::from(0x04));
        assert_eq!(result, Some(vec![
                (Handle::from(0x04), Uuid::Uuid16(0x2902)),
        ]));

        let result = gatt.read(Handle::from(0x03));
        assert_eq!(result, Some(AttributeValue::Value(Bytes::from("MYDEVICENAME"))));
    }

}
