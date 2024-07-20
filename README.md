# Yew translator

> A i18n implementation for yew using a string templater.

Based on [Yew I18n](https://crates.io/crates/yew-i18n), this implementation has slight differences and is making use of a custom string templater parser.

## How to use

When importing `yew_translator` in your project, consider only using the `yew-i18n` like this:
```toml
[dependencies]
yew_translator = { version = "0.1", default-features = false, features = ["yew-i18n"] }
```

This allows to remove the entire `translation_templater` access that you won't need, except if you want to create your own `i18n` for something else than `yew` (for this case, you can remove the `yew-i18n` feature if you want and you'll get rid of `yew` from this crate).

Now, create your yew component and initialize your `i18n` inside it:
```rs
use yew_translator::*;
...
let supported_languages = vec!["en", "fr"];
let mut translations: HashMap<String, serde_json::Value> = HashMap::new(); // Import your own translations (language -> transation JSON)
html!(
  <I18nProvider {supported_languages} {translations}>
    <WhateverYouWant />
  </I18nProvider>
)
```

Finaly, in a child component, use the hook `use_translation()` to handle translations like in [Yew I18n](https://crates.io/crates/yew-i18n) (some field names may vary).

By default, `en` and `fr` are in the field `supported_languages`, but you can change this by inserting your own language codes. 
For the field `translations`, you must have a hashmap containing the language associated with the `JSON` containing your translations.
Finaly, you can use the field `current_language` to set your own default language used by `i18n`. By default, `current_language` is set to `en`.

## JSON translations

You can write your JSON using the system of `key`: `value` using the dot notation to mark the child access (even on array).
So for a JSON such as:
```json
{
  "root": {
    "child_a": ["elem_0", "elem_1", "elem_2", {
      "name": "Janne"
    }],
    "child_b": {
      "first_name": "John",
      "last_name": "Doe",
      "age": 25
    }
  }
}
```
You'll get all the following paths:
- `root.child_a.0`
- `root.child_a.1`
- `root.child_a.2`
- `root.child_a.3.name`
- `root.child_b.first_name`
- `root.child_b.last_name`
- `root.child_b.age`

## Templates and Data

Following the `t` method used by `yew-i18n`, you'll find the method `tt` where you'll put the key of the value you want and you'll insert data to inject in the template found from the key.
Now, you'll treat all your values as templates where you'll be able to do some injections.
The syntax is the following:
- `{{data_field_name}}`: inject a value from your data in the template. (No parsing)
- `{{*data_field_name}}`: use the value of your data as a key of your translations to inject it's value. (No parsing)
- `{{{translation_field_name}}}`: use the value of your translation key to inject it's template, forcing you to also inject the needed values. (Parsing happen)
- `{{{*data_field_name}}}`: use the value of your data as a key of your translations to inject it's template, forcing you to also inject the needed values. (Parsing happen)
- `{{{**data_field_name}}}`: use the value of your data as a template for your translations, helping with the creation of dynamic templates using references. (Parsing happen)

Here's some rules to also follow:
- The `\` symbol followed by `{`, `}` or `\` will always escape the next character, making `\` ignored in the output.
- When parsing the key name, the symbol `\` followed by `*` will result in the character `*` being outputed.
- Escaping the `*` symbol is only useful right after a `{{` or `{{{`.
- You can have as many template as you want inside other template, but be careful of infinite loops.
