pub type OverrideMessage = Box<dyn Fn(&String) -> String>;

/// Options for the string templater.
#[derive(Default)]
pub struct StringTemplaterOptions {
    /// If true, `StringTemplaterError`::UnknownField` will never be returned.
    pub safe_parse: bool,
    /// If true, display the missing keys in the data.
    pub display_missing_keys: bool,
    /// Override the message for the missing data field.
    pub override_missing_keys: Option<OverrideMessage>,
    /// If true, display the missing keys in the translation.
    pub display_missing_translations: bool,
    /// Override the message for the missing translations field.
    pub override_missing_translations: Option<OverrideMessage>,
}
