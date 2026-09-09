//! A translator with a default-locale fallback chain.
//!
//! Lookup order for `translate("es-MX", key, ..)`:
//! 1. `"es-MX"` exact match
//! 2. `"es"` language-only match
//! 3. `"en"` default-locale match (if different from the above)
//! 4. The key itself (visible placeholder for missing translations)

use crate::catalog::Catalog;
use crate::locale::Locale;

/// A translator with a default-locale fallback chain.
#[derive(Debug, Clone)]
pub struct Translator {
    catalog: Catalog,
    default: Locale,
}

impl Translator {
    /// Crate-internal constructor from a catalog.
    pub(crate) fn from_catalog(catalog: Catalog, default: Locale) -> Self {
        Self { catalog, default }
    }

    /// Translates `key` for `locale` with `params` interpolated into
    /// `{placeholder}` slots.
    #[must_use]
    pub fn translate(&self, locale: &str, key: &str, params: &[(&str, &str)]) -> String {
        let loc = Locale::new_unchecked(locale);

        if let Some(text) = self.catalog.get(loc.as_str(), key) {
            return Self::interpolate(text, params);
        }
        if let Some(text) = self.catalog.get(loc.language(), key) {
            return Self::interpolate(text, params);
        }
        if loc.as_str() != self.default.as_str() && loc.language() != self.default.language() {
            if let Some(text) = self.catalog.get(self.default.as_str(), key) {
                return Self::interpolate(text, params);
            }
        }
        key.to_string()
    }

    /// Checks if a key exists for a given locale (with fallback chain).
    #[must_use]
    pub fn exists(&self, locale: &str, key: &str) -> bool {
        let loc = Locale::new_unchecked(locale);
        self.catalog.get(loc.as_str(), key).is_some()
            || self.catalog.get(loc.language(), key).is_some()
            || self.catalog.get(self.default.as_str(), key).is_some()
    }

    /// Replaces `{placeholder}` tokens in `template` with `params`.
    fn interpolate(template: &str, params: &[(&str, &str)]) -> String {
        let mut out = String::with_capacity(template.len());
        let mut rest = template;

        while let Some(start) = rest.find(crate::PLACEHOLDER_START) {
            out.push_str(&rest[..start]);
            let after_start = &rest[start + crate::PLACEHOLDER_START.len_utf8()..];

            if let Some(end) = after_start.find(crate::PLACEHOLDER_END) {
                let name = &after_start[..end];
                let replaced = params
                    .iter()
                    .find(|(k, _)| *k == name)
                    .map_or("", |(_, v)| v);
                out.push_str(replaced);
                rest = &after_start[end + crate::PLACEHOLDER_END.len_utf8()..];
            } else {
                out.push(crate::PLACEHOLDER_START);
                rest = after_start;
            }
        }
        out.push_str(rest);
        out
    }
}
