#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

pub use derive_app_properties::AppProperties;

pub trait AppPropertiesExt: Sized {
    type Error;

    /// Loads the properties from their configured source, replacing environment variable references.
    ///
    /// # Errors
    ///
    /// Returns an error if the source could not be deserialized.
    fn load() -> Result<Self, Self::Error>;
}
