use gatt::characteristics as ch;
use gatt::services as srv;
use gatt::{CharacteristicProperties, Registration};

pub(crate) fn add(registration: &mut Registration<super::Token>) {
    // Generic Attirbute
    registration.add_primary_service(srv::GENERIC_ATTRIBUTE);
    // ServiceChanged
    registration.add_characteristic(
        ch::SERVICE_CHANGED,
        vec![0x00],
        CharacteristicProperties::INDICATE,
    );
}
