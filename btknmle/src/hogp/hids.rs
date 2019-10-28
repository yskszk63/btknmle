use super::DatabaseBuilder;
use crate::gatt::model::{Handle, Uuid};
use crate::gatt::{CharacteristicProperties, CCCD};

pub(crate) fn add(builder: &mut DatabaseBuilder) -> (Handle, Handle) {
    builder.begin_service(Uuid::Uuid16(0x1812));
    builder.with_characteristic(
        CharacteristicProperties::READ,
        Uuid::Uuid16(0x2A4A), // HID Information
        vec![0x10, 0x01, 0x00, 0x02],
    );
    builder.with_characteristic(
        CharacteristicProperties::READ,
        Uuid::Uuid16(0x2A4B), // Reportmap
        vec![
            0x05, 0x01, // Usage Page (Generic Desktop)
            0x09, 0x06, // Usage (Keyboard)
            0xa1, 0x01, // Collection (Application)
            0x85, 0x01, // Report ID 0x01
            0x05, 0x07, // Usage Page (Keyboard/Keypad)
            0x19, 0xe0, // UsageMin (0xE0)
            0x29, 0xe7, // UsageMax (0xE7)
            0x14, // LogicalMin (0)
            0x25, 0x01, // LogicalMax (1)
            0x75, 0x01, // Report Size (1)
            0x95, 0x08, // Report Count (8)
            0x81, 0x02, // Input (Rel)
            0x81, 0x03, // Input (Rel Var)
            0x95, 0x05, // Report Count (5)
            0x05, 0x08, // Usage Page (LED)
            0x19, 0x01, // UsageMin (1)
            0x29, 0x05, // UsageMax (5)
            0x91, 0x02, // Output (Rel)
            0x95, 0x01, // Report Count (1)
            0x75, 0x03, // Report Size (3)
            0x91, 0x01, // Output (Array)
            0x95, 0x06, // Report Count (1)
            0x75, 0x08, // Report Size (8)
            0x14, // LogicalMin (0)
            0x26, 0xa4, 0x00, // LogicalMax(0xA400)
            0x05, 0x07, // Usage Page (Keyboard/Keypad)
            0x18, // UsageMin (0)
            0x29, 0xa4, // UsageMax (0xA4)
            0x80, // ?
            0xc0, // End Collection
            0x05, 0x01, // Usage Page (Generic Desktop)
            0x09, 0x02, // Usage (Mouse)
            0xa1, 0x01, // Collection (Application)
            0x85, 0x02, // Report ID
            0x09, 0x01, // Usage (Pointer)
            0xa0, 0x05, 0x09, // Usage Page (?)
            0x19, 0x01, // UsageMin (1)
            0x29, 0x03, // UsageMax (3)
            0x14, // LogicalMin (0)
            0x25, 0x01, // UsageMax (1)
            0x95, 0x03, // Report Count
            0x75, 0x01, // Report Size
            0x81, 0x02, // Input (Rel)
            0x95, 0x01, // Report Count (1)
            0x75, 0x05, // Report Size (5)
            0x81, 0x01, // Input (Var)
            0x05, 0x01, // Usage Page (Generic Desktop)
            0x15, 0x81, // LogicalMin (0x81)
            0x25, 0x7f, // LogicalMax (0x7f)
            0x75, 0x08, // Report Size (8)
            0x95, 0x02, // Report Cont (2)
            0x09, 0x30, // Usage ?
            0x09, 0x31, // Usage ?
            0x81, 0x06, // Input (Rel Wrap)
            0x15, 0x81, // LogicalMin (0x81)
            0x25, 0x7f, // LogicalMax (0x7f)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x09, 0x38, // Usage (?)
            0x81, 0x06, // Input (Rel Wrap)
            0xc0, // End Collection
            0xc0, // End Collection
        ],
    );

    let kbd = builder.with_characteristic(
        CharacteristicProperties::READ | CharacteristicProperties::NOTIFY,
        Uuid::Uuid16(0x2A4D), // Report
        vec![0x10, 0x01, 0x00, 0x00, 0x02],
    );
    builder.with_descriptor(Uuid::Uuid16(0x2908), vec![0x01, 0x01]);
    builder.with_cccd(CCCD::empty());

    let mouse = builder.with_characteristic(
        CharacteristicProperties::READ | CharacteristicProperties::NOTIFY,
        Uuid::Uuid16(0x2A4D), // Report
        vec![0x10, 0x01, 0x00, 0x00, 0x02],
    );
    builder.with_descriptor(Uuid::Uuid16(0x2908), vec![0x02, 0x01]);
    builder.with_cccd(CCCD::empty());

    builder.with_characteristic(
        CharacteristicProperties::READ | CharacteristicProperties::WRITE_WITHOUT_RESPONSE,
        Uuid::Uuid16(0x2A4E), // Protocol Mode
        vec![0x01],
    );

    builder.with_characteristic(
        CharacteristicProperties::WRITE_WITHOUT_RESPONSE,
        Uuid::Uuid16(0x2A4C), // Control Point
        vec![],
    );

    (kbd, mouse)
}
