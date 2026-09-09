#![cfg(feature = "json")]

use i18n_kit::{Catalog, Locale};
use std::collections::BTreeMap;

/// Load translations from a JSON string of `{ "key": "value" }` pairs.
///
/// Uses serde_json behind the `json` feature to deserialize into a
/// `BTreeMap<String, String>`, then bulk-inserts into the catalog.
///
/// # Panics
///
/// Panics if the JSON is invalid (development-time error, not runtime).
#[test]
fn json_load_into_catalog() {
    let json = r#"{
        "greeting": "Hello, {name}!",
        "farewell": "Goodbye!",
        "nav.home": "Home",
        "nav.settings": "Settings"
    }"#;

    let parsed: BTreeMap<String, String> = serde_json::from_str(json).unwrap();
    let mut catalog = Catalog::new();
    catalog.insert_bulk("en", &parsed);

    let t = catalog.into_translator("en");
    assert_eq!(
        t.translate("en", "greeting", &[("name", "World")]),
        "Hello, World!"
    );
    assert_eq!(t.translate("en", "nav.home", &[]), "Home");
    assert_eq!(t.translate("en", "nav.settings", &[]), "Settings");
    assert!(!t.exists("en", "nav.nonexistent"));
}

#[test]
fn multi_locale_json_load() {
    let en = r#"{"greeting": "Hello, {name}!"}"#;
    let es = r#"{"greeting": "¡Hola, {name}!"}"#;

    let mut catalog = Catalog::new();
    for (locale, json) in [("en", en), ("es", es)] {
        let parsed: BTreeMap<String, String> = serde_json::from_str(json).unwrap();
        catalog.insert_bulk(locale, &parsed);
    }

    let t = catalog.into_translator("en");
    assert_eq!(
        t.translate("es", "greeting", &[("name", "Mundo")]),
        "¡Hola, Mundo!"
    );
}

#[test]
fn invalid_json_is_error() {
    let result: Result<BTreeMap<String, String>, _> = serde_json::from_str(r#"{"broken": "#);
    assert!(result.is_err());
}
