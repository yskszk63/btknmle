use btknmle_server::gatt::model::Handle;
use btknmle_server::gatt::{Database, DatabaseBuilder};

mod bas;
mod dis;
mod gap;
mod gatt;
mod hids;

pub fn new() -> (Database, Handle, Handle) {
    let mut builder = Database::builder();

    gap::add(&mut builder);
    gatt::add(&mut builder);
    dis::add(&mut builder);
    bas::add(&mut builder);
    let (kbd, mouse) = hids::add(&mut builder);

    (builder.build(), kbd, mouse)
}
