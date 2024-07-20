use std::collections::HashMap;

use super::{StringTemplaterError, StringTemplaterOptions};

/// Generate the template with options using translation and data.
pub fn generate(
    template_str: &str,
    translation: &HashMap<String, String>,
    data: &HashMap<String, String>,
    option: &StringTemplaterOptions,
) -> Result<String, StringTemplaterError> {
    let mut result = String::new();
    let mut chars = template_str.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' => {
                if let Some('{') = chars.peek() {
                    chars.next();

                    let mut apply_template = false;
                    let mut pointer = false;
                    let mut inject = false;
                    let mut key = String::new();

                    // Templating
                    if let Some('{') = chars.peek() {
                        apply_template = true;
                        chars.next();
                    }

                    // Pointer
                    if let Some('*') = chars.peek() {
                        pointer = true;
                        chars.next();
                        if let Some('*') = chars.peek() {
                            inject = apply_template; // inject can only be true in a template
                            pointer = !apply_template; // If inject, then bye pointer
                            chars.next();
                        }
                    }

                    // Key looking
                    while let Some(&next) = chars.peek() {
                        if next == '}' {
                            break;
                        } else if next == '\\' {
                            chars.next();
                            match chars.peek() {
                                Some('*') => {
                                    chars.next();
                                    key.push('*');
                                }
                                Some('\\') => {
                                    chars.next();
                                    key.push('\\');
                                }
                                Some('{') => {
                                    chars.next();
                                    key.push('{');
                                }
                                Some('}') => {
                                    chars.next();
                                    key.push('}');
                                }
                                _ => key.push('\\'),
                            }
                        } else {
                            key.push(chars.next().unwrap());
                        }
                    }

                    // Error handling
                    if let Some('}') = chars.peek() {
                        chars.next();
                    } else if apply_template {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing three curvy bracket `}}` around `{}`.",
                            key
                        )));
                    } else {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing two curvy bracket `}}` around `{}`.",
                            key
                        )));
                    }
                    if let Some('}') = chars.peek() {
                        chars.next();
                    } else if apply_template {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing two curvy bracket `}}` around `{}`.",
                            key
                        )));
                    } else {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing one curvy bracket `}}` around `{}`.",
                            key
                        )));
                    }
                    if apply_template {
                        if let Some('}') = chars.peek() {
                            chars.next();
                        } else {
                            return Err(StringTemplaterError::MissingCurvyBracket(format!(
                                "Missing one curvy bracket `}}` around `{}`.",
                                key
                            )));
                        }
                    }

                    // Data handling
                    if apply_template {
                        if pointer {
                            if let Some(value) = data.get(&key) {
                                if let Some(value) = translation.get(value) {
                                    match generate(value, translation, data, option) {
                                        Ok(value) => {
                                            result.push_str(&value);
                                        }
                                        Err(err) => return Err(err),
                                    };
                                } else if option.safe_parse {
                                    if option.display_missing_translations {
                                        let patched_value =
                                            option.override_missing_translations.as_ref();
                                        let patched_value =
                                            patched_value.map(|f| f(value)).unwrap_or(format!(
                                                "[MISSING_TRANSLATION_KEY: `{}`]",
                                                value
                                            ));
                                        result.push_str(&patched_value);
                                    }
                                } else {
                                    return Err(StringTemplaterError::UnknownField(format!(
                                        "The field `{}` does not exist in translations.",
                                        key
                                    )));
                                }
                            } else if option.safe_parse {
                                if option.display_missing_keys {
                                    let patched_value = option.override_missing_keys.as_ref();
                                    let patched_value = patched_value
                                        .map(|f| f(&key))
                                        .unwrap_or(format!("[MISSING_DATA_KEY: `{}`]", key));
                                    result.push_str(&patched_value);
                                }
                            } else {
                                return Err(StringTemplaterError::UnknownField(format!(
                                    "The field `{}` does not exist in data.",
                                    key
                                )));
                            }
                        } else if inject {
                            if let Some(value) = data.get(&key) {
                                match generate(value, translation, data, option) {
                                    Ok(value) => {
                                        result.push_str(&value);
                                    }
                                    Err(err) => return Err(err),
                                };
                            } else if option.safe_parse {
                                if option.display_missing_keys {
                                    let patched_value = option.override_missing_keys.as_ref();
                                    let patched_value = patched_value
                                        .map(|f| f(&key))
                                        .unwrap_or(format!("[MISSING_DATA_KEY: `{}`]", key));
                                    result.push_str(&patched_value);
                                }
                            } else {
                                return Err(StringTemplaterError::UnknownField(format!(
                                    "The field `{}` does not exist in data.",
                                    key
                                )));
                            }
                        } else if let Some(value) = translation.get(&key) {
                            match generate(value, translation, data, option) {
                                Ok(value) => {
                                    result.push_str(&value);
                                }
                                Err(err) => return Err(err),
                            };
                        } else if option.safe_parse {
                            if option.display_missing_translations {
                                let patched_value = option.override_missing_translations.as_ref();
                                let patched_value = patched_value
                                    .map(|f| f(&key))
                                    .unwrap_or(format!("[MISSING_TRANSLATION_KEY: `{}`]", key));
                                result.push_str(&patched_value);
                            }
                        } else {
                            return Err(StringTemplaterError::UnknownField(format!(
                                "The field `{}` does not exist in translations.",
                                key
                            )));
                        }
                    } else {
                        // Not template
                        if pointer {
                            // Use the value of a data as a translation key
                            if let Some(value) = data.get(&key) {
                                if let Some(value) = translation.get(value) {
                                    result.push_str(value);
                                } else if option.safe_parse {
                                    if option.display_missing_translations {
                                        let patched_value =
                                            option.override_missing_translations.as_ref();
                                        let patched_value =
                                            patched_value.map(|f| f(value)).unwrap_or(format!(
                                                "[MISSING_TRANSLATION_KEY: `{}`]",
                                                value
                                            ));
                                        result.push_str(&patched_value);
                                    }
                                } else {
                                    return Err(StringTemplaterError::UnknownField(format!(
                                        "The field `{}` does not exist in translations.",
                                        key
                                    )));
                                }
                            } else if option.safe_parse {
                                if option.display_missing_keys {
                                    let patched_value = option.override_missing_keys.as_ref();
                                    let patched_value = patched_value
                                        .map(|f| f(&key))
                                        .unwrap_or(format!("[MISSING_DATA_KEY: `{}`]", key));
                                    result.push_str(&patched_value);
                                }
                            } else {
                                return Err(StringTemplaterError::UnknownField(format!(
                                    "The field `{}` does not exist in data.",
                                    key
                                )));
                            }
                        } else if let Some(value) = data.get(&key) {
                            result.push_str(value);
                        } else if option.safe_parse {
                            if option.display_missing_keys {
                                let patched_value = option.override_missing_keys.as_ref();
                                let patched_value = patched_value
                                    .map(|f| f(&key))
                                    .unwrap_or(format!("[MISSING_DATA_KEY: `{}`]", key));
                                result.push_str(&patched_value);
                            }
                        } else {
                            return Err(StringTemplaterError::UnknownField(format!(
                                "The field `{}` does not exist in data.",
                                key
                            )));
                        }
                    }
                } else {
                    result.push('{');
                }
            }
            '\\' => {
                if let Some(&next) = chars.peek() {
                    if next == '{' {
                        chars.next();
                        result.push('{');
                    } else if next == '}' {
                        chars.next();
                        result.push('}');
                    } else if next == '\\' {
                        chars.next();
                        result.push(c);
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            }
            _ => result.push(c),
        };
    }
    Ok(result)
}
