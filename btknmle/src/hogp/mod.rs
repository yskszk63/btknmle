use btknmle_server::gatt::{Database, DatabaseBuilder};
use btknmle_server::gatt::model::Handle;

mod gatt;
mod gap;
mod dis;
mod bas;
mod hids;

pub fn new() -> (Database, Handle, Handle) {
    let mut builder = Database::builder();

    gatt::add(&mut builder);
    gap::add(&mut builder);
    dis::add(&mut builder);
    bas::add(&mut builder);
    let (kbd, mouse) = hids::add(&mut builder);

    (builder.build(), kbd, mouse)
}
