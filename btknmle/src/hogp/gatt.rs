use super::DatabaseBuilder;
use crate::gatt::model::Uuid;

pub(crate) fn add(builder: &mut DatabaseBuilder) {
    // Generic Attirbute
    builder.begin_service(Uuid::Uuid16(0x1801));
}
