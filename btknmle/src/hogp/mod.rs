use ::gatt::Registration;

mod bas;
mod dis;
mod gap;
mod gatt;
mod hids;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Token {
    Keyboard,
    Mouse,
}

pub(crate) fn new() -> Registration<Token> {
    let mut registration = Registration::new();

    gap::add(&mut registration);
    gatt::add(&mut registration);
    dis::add(&mut registration);
    bas::add(&mut registration);
    hids::add(&mut registration);

    registration
}
