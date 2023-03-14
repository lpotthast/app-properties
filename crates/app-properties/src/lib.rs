#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

pub use derive_app_properties::AppProperties;

pub trait AppPropertiesExt: Sized {
    type Error;

    fn load() -> Result<Self, Self::Error>;
}
