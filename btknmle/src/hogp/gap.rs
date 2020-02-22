use super::DatabaseBuilder;
use crate::gatt::model::Uuid;
use crate::gatt::CharacteristicProperties;

pub(crate) fn add(builder: &mut DatabaseBuilder) {
    builder.begin_service(Uuid::Uuid16(0x1800));

    // Device Name
    builder.with_characteristic(
        CharacteristicProperties::READ,
        Uuid::Uuid16(0x2A00),
        "btknmle",
    );
    // Appearance
    builder.with_characteristic(
        CharacteristicProperties::READ,
        Uuid::Uuid16(0x2A01),
        vec![0xC2, 0x03],
    ); // HID mouse
    // Resolvable Private Address Only
    builder.with_characteristic(
        CharacteristicProperties::READ,
        Uuid::Uuid16(0x2AC9),
        [0].as_ref(),
    );
}
