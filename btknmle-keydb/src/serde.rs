use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::marker::PhantomData;

use btmgmt::packet::{
    Address, AddressType, IdentityResolvingKey, LongTermKey, LongTermKeyBuilder, LongTermKeyType,
};
use serde::de::{Deserialize, Deserializer, Error as _, MapAccess, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
struct WrapperVisitor<T>(PhantomData<T>);

#[derive(Debug)]
pub(crate) struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Wrapper<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(v: T) -> Self {
        Wrapper(v)
    }
}

impl TryFrom<Wrapper<Vec<u8>>> for [u8; 16] {
    type Error = Vec<u8>;

    fn try_from(value: Wrapper<Vec<u8>>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<Wrapper<Vec<u8>>> for [u8; 8] {
    type Error = Vec<u8>;

    fn try_from(value: Wrapper<Vec<u8>>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<'a> Serialize for Wrapper<&'a Address> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_ref().to_string())
    }
}

impl<'de> Deserialize<'de> for Wrapper<Address> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WrapperVisitor::<Address>(PhantomData))
    }
}

impl<'de> Visitor<'de> for WrapperVisitor<Address> {
    type Value = Wrapper<Address>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v = v
            .parse()
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &"address"))?;
        Ok(Wrapper(v))
    }
}

impl<'a> Serialize for Wrapper<&'a AddressType> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let address_type = match self.as_ref() {
            AddressType::BrEdr => "bredr",
            AddressType::LePublic => "public",
            AddressType::LeRandom => "random",
        };
        serializer.serialize_str(address_type)
    }
}

impl<'de> Deserialize<'de> for Wrapper<AddressType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WrapperVisitor::<AddressType>(PhantomData))
    }
}

impl<'de> Visitor<'de> for WrapperVisitor<AddressType> {
    type Value = Wrapper<AddressType>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address type")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let address_type = match v {
            "bredr" => AddressType::BrEdr,
            "public" => AddressType::LePublic,
            "random" => AddressType::LeRandom,
            x => return Err(E::invalid_value(Unexpected::Str(x), &"bredr|public|random")),
        };
        Ok(Wrapper(address_type))
    }
}

impl<'a> Serialize for Wrapper<&'a [u8]> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self
            .as_ref()
            .iter()
            .map(|v| format!("{:02x}", v))
            .collect::<String>();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Wrapper<Vec<u8>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WrapperVisitor::<Vec<u8>>(PhantomData))
    }
}

impl<'de> Visitor<'de> for WrapperVisitor<Vec<u8>> {
    type Value = Wrapper<Vec<u8>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hex")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 2 != 0 {
            return Err(E::invalid_length(v.len(), &"even"));
        }
        let bytes = (0..v.len() / 2)
            .map(|n| n * 2)
            .map(|n| u8::from_str_radix(&v[n..n + 2], 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &"hex string"))?;
        Ok(Wrapper(bytes))
    }
}

impl<'a> Serialize for Wrapper<[u8; 16]> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Wrapper(self.0.as_ref()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Wrapper<[u8; 8]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = deserializer.deserialize_str(WrapperVisitor::<Vec<u8>>(PhantomData))?;
        Ok(Wrapper(v.try_into().map_err(|v: Vec<u8>| {
            D::Error::invalid_length(v.len(), &"8")
        })?))
    }
}

impl<'de> Deserialize<'de> for Wrapper<[u8; 16]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = deserializer.deserialize_str(WrapperVisitor::<Vec<u8>>(PhantomData))?;
        Ok(Wrapper(v.try_into().map_err(|v: Vec<u8>| {
            D::Error::invalid_length(v.len(), &"16")
        })?))
    }
}

impl<'a> Serialize for Wrapper<&'a LongTermKeyType> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let address_type = match self.as_ref() {
            LongTermKeyType::AuthenticatedKey => "authenticated",
            LongTermKeyType::UnauthenticatedKey => "unauthenticated",
            LongTermKeyType::AuthenticatedP256Key => "authenticatedp256",
            LongTermKeyType::UnauthenticatedP256Key => "unauthenticatedp256",
            LongTermKeyType::DebugKeyP256 => "debugp256",
        };
        serializer.serialize_str(address_type)
    }
}

impl<'de> Deserialize<'de> for Wrapper<LongTermKeyType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WrapperVisitor::<LongTermKeyType>(PhantomData))
    }
}

impl<'de> Visitor<'de> for WrapperVisitor<LongTermKeyType> {
    type Value = Wrapper<LongTermKeyType>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address type")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let key_type = match v {
            "authenticated" => LongTermKeyType::AuthenticatedKey,
            "unauthenticated" => LongTermKeyType::UnauthenticatedKey,
            "authenticatedp256" => LongTermKeyType::AuthenticatedP256Key,
            "unauthenticatedp256" => LongTermKeyType::UnauthenticatedP256Key,
            "debugp256" => LongTermKeyType::DebugKeyP256,
            x => return Err(E::invalid_value(
                Unexpected::Str(x),
                &"authenticated|unauthenticated|authenticatedp256|unauthenticatedp256|debugp256",
            )),
        };
        Ok(Wrapper(key_type))
    }
}

impl Serialize for Wrapper<IdentityResolvingKey> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("IdentityResolvingKey", 3)?;
        s.serialize_field("address", &Wrapper(self.0.address()))?;
        s.serialize_field("address_type", &Wrapper(self.0.address_type()))?;
        s.serialize_field("value", &Wrapper(self.0.value().as_ref()))?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Wrapper<IdentityResolvingKey> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "IdentityResolvingKey",
            &["address", "address_type", "value"],
            WrapperVisitor::<IdentityResolvingKey>(PhantomData),
        )
    }
}

impl<'de> Visitor<'de> for WrapperVisitor<IdentityResolvingKey> {
    type Value = Wrapper<IdentityResolvingKey>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "identity resolving key")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut address = None;
        let mut address_type = None;
        let mut value = None;

        while let Some(key) = map.next_key()? {
            match key {
                "address" => {
                    address = Some(map.next_value::<Wrapper<Address>>()?).map(Wrapper::into_inner)
                }
                "address_type" => {
                    address_type =
                        Some(map.next_value::<Wrapper<AddressType>>()?).map(Wrapper::into_inner)
                }
                "value" => {
                    value = Some(map.next_value::<Wrapper<[u8; 16]>>()?).map(Wrapper::into_inner)
                }
                x => {
                    return Err(A::Error::unknown_field(
                        x,
                        &["address", "address_type", "value"],
                    ))
                }
            }
        }

        match (address, address_type, value) {
            (Some(address), Some(address_type), Some(value)) => Ok(Wrapper(
                IdentityResolvingKey::new(address, address_type, value),
            )),
            (None, _, _) => Err(A::Error::missing_field("address")),
            (_, None, _) => Err(A::Error::missing_field("address_type")),
            (_, _, None) => Err(A::Error::missing_field("value")),
        }
    }
}

impl Serialize for Wrapper<LongTermKey> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LongTermKey", 8)?;
        s.serialize_field("address", &Wrapper(self.0.address()))?;
        s.serialize_field("address_type", &Wrapper(self.0.address_type()))?;
        s.serialize_field("key_type", &Wrapper(self.0.key_type()))?;
        s.serialize_field("master", self.0.master())?;
        s.serialize_field("encryption_size", self.0.encryption_size())?;
        s.serialize_field("encryption_diversifier", self.0.encryption_diversifier())?;
        s.serialize_field("random_number", &Wrapper(self.0.random_number().as_ref()))?;
        s.serialize_field("value", &Wrapper(self.0.value().as_ref()))?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Wrapper<LongTermKey> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "LongTermKey",
            &[
                "address",
                "address_type",
                "key_type",
                "master",
                "encryption_size",
                "encryption_diversifier",
                "random_number",
                "value",
            ],
            WrapperVisitor::<LongTermKey>(PhantomData),
        )
    }
}

impl<'de> Visitor<'de> for WrapperVisitor<LongTermKey> {
    type Value = Wrapper<LongTermKey>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "long term key")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut address = None;
        let mut address_type = None;
        let mut key_type = None;
        let mut master = None;
        let mut encryption_size = None;
        let mut encryption_diversifier = None;
        let mut random_number = None;
        let mut value = None;

        while let Some(key) = map.next_key()? {
            match key {
                "address" => address = Some(map.next_value::<Wrapper<Address>>()?.into_inner()),
                "address_type" => {
                    address_type = Some(map.next_value::<Wrapper<AddressType>>()?.into_inner())
                }
                "key_type" => {
                    key_type = Some(map.next_value::<Wrapper<LongTermKeyType>>()?.into_inner())
                }
                "master" => master = Some(map.next_value::<bool>()?),
                "encryption_size" => encryption_size = Some(map.next_value::<u8>()?),
                "encryption_diversifier" => encryption_diversifier = Some(map.next_value::<u16>()?),
                "random_number" => {
                    random_number = Some(map.next_value::<Wrapper<[u8; 8]>>()?.into_inner())
                }
                "value" => value = Some(map.next_value::<Wrapper<[u8; 16]>>()?.into_inner()),
                x => return Err(A::Error::unknown_field(x, &["address"])),
            }
        }

        match (
            address,
            address_type,
            key_type,
            master,
            encryption_size,
            encryption_diversifier,
            random_number,
            value,
        ) {
            (
                Some(address),
                Some(address_type),
                Some(key_type),
                Some(master),
                Some(encryption_size),
                Some(encryption_diversifier),
                Some(random_number),
                Some(value),
            ) => LongTermKeyBuilder::default()
                .address(address)
                .address_type(address_type)
                .key_type(key_type)
                .master(master)
                .encryption_size(encryption_size)
                .encryption_diversifier(encryption_diversifier)
                .random_number(random_number)
                .value(value)
                .build()
                .map(Wrapper)
                .map_err(A::Error::custom),
            (None, _, _, _, _, _, _, _) => Err(A::Error::missing_field("address")),
            (_, None, _, _, _, _, _, _) => Err(A::Error::missing_field("address_type")),
            (_, _, None, _, _, _, _, _) => Err(A::Error::missing_field("key_type")),
            (_, _, _, None, _, _, _, _) => Err(A::Error::missing_field("master")),
            (_, _, _, _, None, _, _, _) => Err(A::Error::missing_field("encryption_size")),
            (_, _, _, _, _, None, _, _) => Err(A::Error::missing_field("encryption_diversifier")),
            (_, _, _, _, _, _, None, _) => Err(A::Error::missing_field("random_number")),
            (_, _, _, _, _, _, _, None) => Err(A::Error::missing_field("value")),
        }
    }
}
