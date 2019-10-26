use super::DatabaseBuilder;
use crate::gatt::model::Uuid;
use crate::gatt::CharacteristicProperties;

pub(crate) fn add(builder: &mut DatabaseBuilder) {
    builder.begin_service(Uuid::Uuid16(0x180A));
    builder.with_characteristic(
        CharacteristicProperties::READ,
        Uuid::Uuid16(0x2A29),
        "MYMANUFACTURE",
    );
    builder.with_characteristic(CharacteristicProperties::READ, Uuid::Uuid16(0x2A24), "1234");
    builder.with_characteristic(CharacteristicProperties::READ, Uuid::Uuid16(0x2A24), "9999");
}
