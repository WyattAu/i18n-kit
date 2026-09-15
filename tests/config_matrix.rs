//! Config-knob behavior matrix for i18n-kit.
//!
//! Knobs: the catalog contents, the `default_locale` handed to
//! `into_translator`, and the interpolation `params`. Each must
//! observably change translation output.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use i18n_kit::Catalog;
use std::collections::BTreeMap;

fn catalog() -> Catalog {
    let mut c = Catalog::new();
    // en: full translations; es: language-level; es-MX deliberately MISSING
    // "farewell" so the default-locale fallback path is observable.
    c.insert("en", "greeting", "Hello, {name}!");
    c.insert("en", "farewell", "Goodbye, {name}!");
    c.insert("es", "greeting", "Hola, {name}!");
    c
}

// --- default_locale knob ----------------------------------------------------

#[test]
fn knob_default_locale_changes_fallback_target() {
    // es-MX misses "farewell" in both es and es-MX -> falls back to default.
    let en_default = catalog().into_translator("en");
    assert_eq!(
        en_default.translate("es-MX", "farewell", &[("name", "Ana")]),
        "Goodbye, Ana!",
        "missing key must fall back to the default locale"
    );

    // Same catalog, different default locale: the fallback text changes.
    let mut c = catalog();
    c.insert("fr", "farewell", "Adieu, {name}!");
    let fr_default = c.into_translator("fr");
    assert_eq!(
        fr_default.translate("es-MX", "farewell", &[("name", "Ana")]),
        "Adieu, Ana!",
        "changing default_locale must change the fallback output"
    );
}

#[test]
fn knob_fallback_chain_order_is_exact_then_language_then_default_then_key() {
    let t = catalog().into_translator("en");

    // 1. exact locale match (es -> "Hola").
    assert_eq!(t.translate("es", "greeting", &[("name", "X")]), "Hola, X!");

    // 2. language match: es-MX has nothing -> falls to "es".
    assert_eq!(
        t.translate("es-MX", "greeting", &[("name", "X")]),
        "Hola, X!"
    );

    // 3. default locale: es-MX "farewell" missing everywhere relevant -> en.
    assert_eq!(
        t.translate("es-MX", "farewell", &[("name", "X")]),
        "Goodbye, X!"
    );

    // 4. key passthrough: nothing anywhere has "missing.key".
    assert_eq!(
        t.translate("es-MX", "missing.key", &[("name", "X")]),
        "missing.key"
    );
}

// --- catalog contents knob ---------------------------------------------------

#[test]
fn knob_catalog_entry_changes_output_and_overwrite_takes_effect() {
    let t1 = catalog().into_translator("en");
    assert_eq!(
        t1.translate("en", "greeting", &[("name", "Bea")]),
        "Hello, Bea!"
    );

    let mut c2 = catalog();
    c2.insert("en", "greeting", "Howdy, {name}!");
    let t2 = c2.into_translator("en");
    assert_eq!(
        t2.translate("en", "greeting", &[("name", "Bea")]),
        "Howdy, Bea!",
        "inserting/overwriting a catalog entry must change output"
    );
}

#[test]
fn knob_bulk_insert_matches_individual_inserts() {
    let mut map = BTreeMap::new();
    map.insert("a".to_string(), "1".to_string());
    map.insert("b".to_string(), "2".to_string());

    let mut bulk = Catalog::new();
    bulk.insert_bulk("en", &map);
    let t_bulk = bulk.into_translator("en");

    let mut one = Catalog::new();
    one.insert("en", "a", "1");
    one.insert("en", "b", "2");
    let t_one = one.into_translator("en");

    assert_eq!(
        t_bulk.translate("en", "a", &[]),
        t_one.translate("en", "a", &[])
    );
    assert!(t_bulk.exists("en", "b"));
}

// --- params knob --------------------------------------------------------------

#[test]
fn knob_params_change_interpolation_output() {
    let t = catalog().into_translator("en");

    assert_eq!(
        t.translate("en", "greeting", &[("name", "World")]),
        "Hello, World!"
    );
    assert_eq!(
        t.translate("en", "greeting", &[("name", "Rust")]),
        "Hello, Rust!",
        "changing a param value must change the output"
    );
    // Missing param renders as empty slot (documented behavior).
    assert_eq!(t.translate("en", "greeting", &[]), "Hello, !");
}

#[test]
fn knob_exists_reflects_catalog_and_fallback() {
    let t = catalog().into_translator("en");

    assert!(t.exists("en", "greeting"));
    // es-MX has no "greeting" itself, but language + default fallback finds it.
    assert!(t.exists("es-MX", "greeting"));
    assert!(!t.exists("en", "missing.key"));
}
