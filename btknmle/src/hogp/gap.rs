use super::DatabaseBuilder;
use crate::gatt::{CharacteristicProperties, CCCD};
use crate::gatt::model::Uuid;

pub(crate) fn add(builder: &mut DatabaseBuilder) {
    builder.begin_service(Uuid::Uuid16(0x1801));
    builder.with_characteristic(
        CharacteristicProperties::INDICATE,
        Uuid::Uuid16(0x2A05),
        "",
    );
    builder.with_user_description("HELLO WORLD!".into());
    builder.with_cccd(CCCD::empty());
}
