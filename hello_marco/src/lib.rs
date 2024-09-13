pub trait HelloMarco {
    fn hello_marco();
}

// has the same use path as `trait Hello`
// When used in #[derive(HelloMarco)], refers to this type
#[cfg(feature = "derive")]
pub use ::hello_marco_derive::HelloMarco;
