use gatt::characteristics as ch;
use gatt::services as srv;
use gatt::{CharacteristicProperties, Registration};

pub(crate) fn add(registration: &mut Registration<super::Token>) {
    registration.add_primary_service(srv::GENERIC_ACCESS);

    // Device Name
    registration.add_characteristic(ch::DEVICE_NAME, "btknmle", CharacteristicProperties::READ);
    // Appearance [HID generic]
    registration.add_characteristic(
        ch::APPEARANCE,
        vec![0xC0, 0x03],
        CharacteristicProperties::READ,
    );
}
