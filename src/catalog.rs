use crate::locale::Locale;
use std::collections::BTreeMap;

/// A translation catalog mapping locale → key → translated string.
///
/// Keys use dot notation: `"nav.home"`, `"settings.title"`, `"errors.404"`.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    entries: BTreeMap<String, BTreeMap<String, String>>,
}

impl Catalog {
    /// Creates an empty catalog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a translation for `locale`/`key`.
    ///
    /// Locale is normalized (BCP 47 casing). Later inserts overwrite.
    pub fn insert(&mut self, locale: &str, key: &str, value: &str) {
        let loc = Locale::new_unchecked(locale);
        self.entries
            .entry(loc.as_str().to_string())
            .or_default()
            .insert(key.to_string(), value.to_string());
    }

    /// Bulk-inserts translations from a flat map (e.g. parsed JSON/YAML).
    pub fn insert_bulk(&mut self, locale: &str, translations: &BTreeMap<String, String>) {
        for (key, value) in translations {
            self.insert(locale, key, value);
        }
    }

    /// Looks up a translation without fallback. Returns `None` if missing.
    #[must_use]
    pub fn get(&self, locale: &str, key: &str) -> Option<&str> {
        let loc = Locale::new_unchecked(locale);
        self.entries
            .get(loc.as_str())
            .and_then(|keys| keys.get(key))
            .map(|s| s.as_str())
    }

    /// Returns the number of locales in the catalog.
    #[must_use]
    pub fn locale_count(&self) -> usize {
        self.entries.len()
    }

    /// Returns the number of keys for a given locale.
    #[must_use]
    pub fn key_count(&self, locale: &str) -> usize {
        let loc = Locale::new_unchecked(locale);
        self.entries.get(loc.as_str()).map_or(0, |keys| keys.len())
    }

    /// Lists all locales present in the catalog (sorted).
    #[must_use]
    pub fn locales(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }

    /// Consumes the catalog and returns a [`crate::Translator`] with
    /// `default_locale` as the fallback language.
    #[must_use]
    pub fn into_translator(self, default_locale: &str) -> crate::Translator {
        crate::Translator::from_catalog(self, Locale::new_unchecked(default_locale))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut c = Catalog::new();
        c.insert("en", "greeting", "Hello");
        assert_eq!(c.get("en", "greeting"), Some("Hello"));
    }

    #[test]
    fn locale_normalization() {
        let mut c = Catalog::new();
        c.insert("EN-us", "greeting", "Howdy");
        // Normalized to "en-US".
        assert!(c.get("en-US", "greeting").is_some());
        assert!(c.get("en-us", "greeting").is_some());
    }

    #[test]
    fn missing_returns_none() {
        let c = Catalog::new();
        assert!(c.get("en", "greeting").is_none());
    }

    #[test]
    fn bulk_insert() {
        let mut c = Catalog::new();
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        c.insert_bulk("en", &map);
        assert_eq!(c.key_count("en"), 2);
    }

    #[test]
    fn metadata() {
        let mut c = Catalog::new();
        c.insert("en", "a", "1");
        c.insert("en", "b", "2");
        c.insert("es", "a", "1");
        assert_eq!(c.locale_count(), 2);
        assert_eq!(c.key_count("en"), 2);
        assert_eq!(c.key_count("es"), 1);
        assert_eq!(c.locales(), vec!["en", "es"]);
    }
}
