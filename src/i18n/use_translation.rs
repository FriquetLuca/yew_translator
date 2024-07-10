use super::I18nHandler;
use yew::{hook, use_context};

/// Hook to use the I18nHandler.
#[hook]
pub fn use_translation() -> I18nHandler {
    use_context::<I18nHandler>().expect("No I18n handle context provided")
}
