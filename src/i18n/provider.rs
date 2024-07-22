#[cfg(feature = "translation_templater")]
use crate::templater::{
    encode_json_to_hashmap, generate, parse_to_hashmap, StringTemplaterError,
    StringTemplaterOptions,
};
#[cfg(feature = "handlebars")]
use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use std::collections::HashMap;
use yew::{function_component, html, use_state, Callback, ContextProvider, Html, Properties};

/// The I18nHandler struct representing the state and methods for internationalization.
#[derive(Clone, Debug, PartialEq)]
pub struct I18nHandler {
    /// The current language code for translations.
    current_language: String,
    /// List of supported languages.
    supported_languages: Vec<&'static str>,
    /// Set the current language code for translations.
    set_language: Callback<String>,
    /// Translations for different languages, represented as a mapping from language codes to JSON values.
    translations: HashMap<String, HashMap<String, String>>,
}

impl I18nHandler {
    /// Get the current language code for translations.
    pub fn current_language(&self) -> String {
        self.current_language.clone()
    }

    /// Get the supported languages codes for translations.
    pub fn supported_languages(&self) -> Vec<&'static str> {
        self.supported_languages.clone()
    }

    /// Set the current language code for translations.
    pub fn set_language(&self, language: impl Into<String>) -> Result<(), String> {
        let language = language.into();
        if self.supported_languages.contains(&language.as_str()) {
            self.set_language.emit(language);
            Ok(())
        } else {
            Err(format!("The language `{}` is not available", language))
        }
    }

    // Find the value to display for the current language code in use.
    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(&self.current_language)
            .and_then(|language_json| language_json.get(key))
            .map_or_else(
                || {
                    format!(
                        "['{}'](T - '{}')",
                        self.current_language.to_uppercase(),
                        key
                    )
                },
                |s| s.clone(),
            )
    }

    #[cfg(feature = "translation_templater")]
    // Find the template to display for the current language code in use and inject it some data (Use the translation_templater).
    pub fn tt<T: ?Sized + Serialize>(&self, key: &str, data: &T) -> String {
        match parse_to_hashmap(data) {
            Ok(data) => self.tth_with_options(
                key,
                &data,
                &StringTemplaterOptions {
                    safe_parse: true,
                    display_missing_keys: true,
                    override_missing_keys: {
                        let current_language = self.current_language.clone();
                        Some(Box::new(move |key| {
                            format!("['{}'](D - '{}')", current_language.to_uppercase(), key)
                        }))
                    },
                    display_missing_translations: true,
                    override_missing_translations: {
                        let current_language = self.current_language.clone();
                        Some(Box::new(move |key| {
                            format!("['{}'](T - '{}')", current_language.to_uppercase(), key)
                        }))
                    },
                },
            ),
            Err(err) => err.to_string(),
        }
    }

    #[cfg(feature = "translation_templater")]
    // Find the template to display for the current language code in use and inject it some data (Use the translation_templater).
    pub fn tth(&self, key: &str, data: &HashMap<String, String>) -> String {
        self.tth_with_options(
            key,
            data,
            &StringTemplaterOptions {
                safe_parse: true,
                display_missing_keys: true,
                override_missing_keys: {
                    let current_language = self.current_language.clone();
                    Some(Box::new(move |key| {
                        format!("['{}'](D - '{}')", current_language.to_uppercase(), key)
                    }))
                },
                display_missing_translations: true,
                override_missing_translations: {
                    let current_language = self.current_language.clone();
                    Some(Box::new(move |key| {
                        format!("['{}'](T - '{}')", current_language.to_uppercase(), key)
                    }))
                },
            },
        )
    }

    #[cfg(feature = "translation_templater")]
    // Find the template to display for the current language code in use and inject it some data (Use the translation_templater).
    pub fn tth_with_options(
        &self,
        key: &str,
        data: &HashMap<String, String>,
        option: &StringTemplaterOptions,
    ) -> String {
        self.translations
            .get(&self.current_language)
            .map(|language_json| {
                let result = match language_json.get(key) {
                    Some(template_string) => generate(template_string, language_json, data, option),
                    None => Err(StringTemplaterError::UnknownField(format!(
                        "The field `{}` does not exist in the hashmap.",
                        key
                    ))),
                };
                match result {
                    Ok(r) => r,
                    Err(err) => err.to_string(),
                }
            })
            .map_or_else(
                || {
                    format!(
                        "['{}'](T - '{}')",
                        self.current_language.to_uppercase(),
                        key
                    )
                },
                |s| s.clone(),
            )
    }

    #[cfg(feature = "translation_templater")]
    // Find the template to display for the current language code in use and inject it some data (Use the translation_templater).
    pub fn tt_with_options<T: ?Sized + Serialize>(
        &self,
        key: &str,
        data: &T,
        option: &StringTemplaterOptions,
    ) -> String {
        match parse_to_hashmap(data) {
            Ok(data) => self.tth_with_options(key, &data, option),
            Err(err) => err.to_string(),
        }
    }

    #[cfg(feature = "handlebars")]
    // Using your own instance of Handlebars, find the template to display for the current language code in use and inject it some data (Use handlebars).
    pub fn thb_registry<T: Serialize>(
        &self,
        reg: Handlebars,
        key: &str,
        data: &T,
    ) -> Result<String, RenderError> {
        reg.render_template(&self.t(key), data)
    }

    #[cfg(feature = "handlebars")]
    // Find the template to display for the current language code in use and inject it some data (Use handlebars).
    pub fn thb<T: Serialize>(&self, key: &str, data: &T) -> Result<String, RenderError> {
        let reg = Handlebars::new();
        reg.render_template(&self.t(key), data)
    }
}

/// Configuration for the YewI18nProvider component.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct I18nProviderProps {
    /// List of supported languages. Defaults to English and French if not specified.
    #[prop_or_else(|| vec!["en", "fr"])]
    pub supported_languages: Vec<&'static str>,
    /// Translations for different languages, represented as a mapping from language codes to JSON values.
    #[prop_or_default]
    pub translations: HashMap<String, serde_json::Value>,
    /// The current language code for translations, English if not specified.
    #[prop_or(String::from("en"))]
    pub current_language: String,
    /// The child components to be wrapped with the YewI18n context.
    pub children: Html,
}

/// Yew component for providing the YewI18n context to its children.
#[function_component]
pub fn I18nProvider(props: &I18nProviderProps) -> Html {
    let translations = use_state(|| props.translations.clone());
    let current_language = use_state(|| props.current_language.clone());
    let supported_languages = use_state(|| props.supported_languages.clone());

    let set_language = {
        let current_language = current_language.clone();
        Callback::from(move |language: String| current_language.set(language.clone()))
    };
    // |language_json| encode_json_to_hashmap(language_json)
    let i18n_handle = I18nHandler {
        translations: (*translations)
            .clone()
            .iter()
            .map(|(key, value)| (key.clone(), encode_json_to_hashmap(value)))
            .collect(),
        set_language,
        supported_languages: (*supported_languages).clone(),
        current_language: (*current_language).clone(),
    };
    html!(<ContextProvider<I18nHandler> context={i18n_handle.clone()}>{ props.children.clone() }</ContextProvider<I18nHandler>>)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde::Serialize;
    use yew::Callback;

    use crate::*;

    #[derive(Debug, Serialize)]
    struct Person {
        pub name: String,
        pub childs: Option<Vec<Person>>,
        pub template: Option<String>,
    }

    fn default_en_translation() -> HashMap<String, String> {
        let mut hashmap = HashMap::new();
        hashmap.insert("key".to_string(), "Value.".to_string());
        hashmap.insert(
            "hard_hello".to_string(),
            "This is hard to say but... {{{hello}}}..".to_string(),
        );
        hashmap.insert("hello".to_string(), "Hello {{name}}.".to_string());
        hashmap.insert("hijack_value".to_string(), "Hello {{*name}}.".to_string());
        hashmap.insert(
            "useless_template".to_string(),
            "You're named {{childs.0.name}}. Crazy isn't it?".to_string(),
        );
        hashmap.insert(
            "hijack_template_hello".to_string(),
            "Hear me out! {{{*name}}} :P".to_string(),
        );
        hashmap.insert(
            "inject_template_for_array".to_string(),
            "Here's the family:\n{{{**template}}}".to_string(),
        );
        hashmap.insert("handlebars_1".to_string(), "Hello {{name}}".to_string());
        hashmap
    }

    #[cfg(feature = "translation_templater")]
    fn default_translation() -> HashMap<String, HashMap<String, String>> {
        let mut translations = HashMap::new();
        translations.insert("en".to_string(), default_en_translation());
        translations
    }

    #[test]
    #[cfg(feature = "translation_templater")]
    fn test_key_translation() {
        let handler = I18nHandler {
            current_language: "en".to_string(),
            supported_languages: vec!["en"],
            set_language: Callback::noop(),
            translations: default_translation(),
        };
        assert_eq!(handler.t("key"), "Value.".to_string())
    }

    #[test]
    #[cfg(feature = "translation_templater")]
    fn test_template_translation() {
        let handler = I18nHandler {
            current_language: "en".to_string(),
            supported_languages: vec!["en"],
            set_language: Callback::noop(),
            translations: default_translation(),
        };
        let john = Person {
            name: "John".to_string(),
            childs: None,
            template: None,
        };
        assert_eq!(handler.tt("hello", &john), "Hello John.".to_string())
    }

    #[test]
    #[cfg(feature = "translation_templater")]
    fn test_template_nested_translation() {
        let handler = I18nHandler {
            current_language: "en".to_string(),
            supported_languages: vec!["en"],
            set_language: Callback::noop(),
            translations: default_translation(),
        };
        let john = Person {
            name: "John".to_string(),
            childs: None,
            template: None,
        };
        assert_eq!(
            handler.tt("hard_hello", &john),
            "This is hard to say but... Hello John...".to_string()
        )
    }

    #[test]
    #[cfg(feature = "translation_templater")]
    fn test_template_value_pointer_translation() {
        let handler = I18nHandler {
            current_language: "en".to_string(),
            supported_languages: vec!["en"],
            set_language: Callback::noop(),
            translations: default_translation(),
        };
        let john = Person {
            name: "key".to_string(),
            childs: None,
            template: None,
        };
        assert_eq!(
            handler.tt("hijack_value", &john),
            "Hello Value..".to_string()
        )
    }

    #[test]
    #[cfg(feature = "translation_templater")]
    fn test_template_value_template_pointer_translation() {
        let handler = I18nHandler {
            current_language: "en".to_string(),
            supported_languages: vec!["en"],
            set_language: Callback::noop(),
            translations: default_translation(),
        };
        let john = Person {
            name: "useless_template".to_string(),
            template: None,
            childs: Some(vec![Person {
                name: "Janne".to_string(),
                childs: None,
                template: None,
            }]),
        };
        assert_eq!(
            handler.tt("hijack_template_hello", &john),
            "Hear me out! You're named Janne. Crazy isn't it? :P".to_string()
        )
    }

    #[test]
    #[cfg(feature = "translation_templater")]
    fn test_template_value_template_injector_translation() {
        let handler = I18nHandler {
            current_language: "en".to_string(),
            supported_languages: vec!["en"],
            set_language: Callback::noop(),
            translations: default_translation(),
        };
        let childrens = vec![
            Person {
                name: "Janne".to_string(),
                childs: None,
                template: None,
            },
            Person {
                name: "Alice".to_string(),
                childs: None,
                template: None,
            },
            Person {
                name: "Bob".to_string(),
                childs: None,
                template: None,
            },
        ];
        let mut template_content = String::new();
        for i in 0..childrens.len() {
            template_content.push_str("- {{");
            template_content.push_str(&format!("childs.{}.name", i));
            template_content.push_str("}}\n");
        }
        let john = Person {
            name: "John".to_string(),
            template: Some(template_content),
            childs: Some(childrens),
        };
        assert_eq!(
            handler.tt("inject_template_for_array", &john),
            "Here's the family:\n- Janne\n- Alice\n- Bob\n".to_string()
        )
    }

    #[test]
    #[cfg(feature = "handlebars")]
    fn test_template_handlebars() {
        use serde_json::json;
        let handler = I18nHandler {
            current_language: "en".to_string(),
            supported_languages: vec!["en"],
            set_language: Callback::noop(),
            translations: default_translation(),
        };
        let result = match handler.thb("handlebars_1", &json!({"name": "foo"})) {
            Ok(result) => result,
            Err(_) => "".to_string(),
        };
        assert_eq!(result, "Hello foo".to_string())
    }
}
