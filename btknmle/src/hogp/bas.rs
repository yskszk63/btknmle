use super::DatabaseBuilder;
use crate::gatt::model::Uuid;
use crate::gatt::{CharacteristicProperties, CCCD};

pub(crate) fn add(builder: &mut DatabaseBuilder) {
    builder.begin_service(Uuid::Uuid16(0x180F));
    builder.with_characteristic(
        CharacteristicProperties::INDICATE,
        Uuid::Uuid16(0x2A19),
        vec![100],
    );
    builder.with_cccd(CCCD::empty());
}
