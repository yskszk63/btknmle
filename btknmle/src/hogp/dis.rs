use gatt::characteristics as ch;
use gatt::services as srv;
use gatt::{CharacteristicProperties, Registration};

pub(crate) fn add(registration: &mut Registration<super::Token>) {
    registration.add_primary_service(srv::DEVICE_INFORMATION);
    registration.add_characteristic(
        ch::MANUFACTURER_NAME_STRING,
        "MYMANUFACTURE",
        CharacteristicProperties::READ,
    );
    registration.add_characteristic(
        ch::MODEL_NUMBER_STRING,
        "1234",
        CharacteristicProperties::READ,
    );
    registration.add_characteristic(
        ch::SERIAL_NUMBER_STRING,
        "9999",
        CharacteristicProperties::READ,
    );
}
