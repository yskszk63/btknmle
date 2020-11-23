use gatt::characteristics as ch;
use gatt::services as srv;
use gatt::{CharacteristicProperties, Registration};

pub(crate) fn add(registration: &mut Registration<super::Token>) {
    registration.add_primary_service(srv::BATTERY);

    registration.add_characteristic(
        ch::BATTERY_LEVEL,
        vec![100],
        CharacteristicProperties::INDICATE,
    );
}
