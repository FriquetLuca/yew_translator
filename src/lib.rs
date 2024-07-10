#[cfg(feature = "yew-i18n")]
mod i18n;
mod templater;

#[cfg(feature = "yew-i18n")]
pub use i18n::*;
#[cfg(feature = "translation_templater")]
pub use templater::*;
