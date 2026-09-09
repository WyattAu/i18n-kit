use crate::error::I18nError;

/// A BCP 47 language tag (e.g. `"en"`, `"en-US"`, `"zh-Hans"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale(String);

impl Locale {
    /// Creates a locale from a string, validating the BCP 47 structure.
    ///
    /// Accepts `language[-script][-region]` where language is 2–3 lowercase
    /// letters, script is 4 letters title-case, and region is 2 letters or
    /// 3 digits uppercase.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::InvalidLocale`] if the tag is empty, too long,
    /// or contains invalid characters.
    pub fn parse(s: &str) -> Result<Self, I18nError> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(I18nError::InvalidLocale("empty locale tag".into()));
        }
        if trimmed.len() > 35 {
            return Err(I18nError::InvalidLocale(format!(
                "locale tag exceeds 35 chars: {trimmed}"
            )));
        }
        if !trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(I18nError::InvalidLocale(format!(
                "locale tag contains invalid characters: {trimmed}"
            )));
        }

        let normalized = Self::normalize(trimmed);
        // Language subtag must be 2–3 letters.
        let lang = normalized.split('-').next().unwrap_or("");
        if lang.len() < 2 || lang.len() > 3 || !lang.chars().all(|c| c.is_ascii_lowercase()) {
            return Err(I18nError::InvalidLocale(format!(
                "invalid language subtag: {lang}"
            )));
        }

        Ok(Self(normalized))
    }

    /// Creates a locale without validation. The caller guarantees the tag
    /// is a well-formed BCP 47 identifier.
    #[must_use]
    pub fn new_unchecked(s: &str) -> Self {
        Self(Self::normalize(s))
    }

    /// Normalizes: lowercase the whole tag, then title-case script and
    /// uppercase region subtags.
    fn normalize(s: &str) -> String {
        let parts: Vec<&str> = s.split('-').collect();
        let mut out = String::with_capacity(s.len());
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                out.push('-');
            }
            if i == 0 {
                // Language: lowercase.
                out.push_str(&part.to_lowercase());
            } else if part.len() == 4 {
                // Script: title-case.
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    out.extend(first.to_uppercase());
                    out.extend(chars.flat_map(|c| c.to_lowercase()));
                }
            } else if part.len() == 2 || part.chars().all(|c| c.is_ascii_digit()) {
                // Region: uppercase.
                out.push_str(&part.to_uppercase());
            } else {
                // Variants/extensions: lowercase.
                out.push_str(&part.to_lowercase());
            }
        }
        out
    }

    /// Returns the normalized locale string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the language subtag (first component, lowercase).
    #[must_use]
    pub fn language(&self) -> &str {
        self.0.split('-').next().unwrap_or("en")
    }

    /// Returns true if this locale matches the given language prefix.
    ///
    /// `"en-US".matches_language("en")` → true.
    #[must_use]
    pub fn matches_language(&self, lang: &str) -> bool {
        self.language() == lang.to_lowercase()
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for Locale {
    type Err = I18nError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_locales() {
        for tag in &["en", "en-US", "zh-Hans", "es-419", "pt-BR", "de-DE-1901"] {
            assert!(Locale::parse(tag).is_ok(), "{tag} should parse");
        }
    }

    #[test]
    fn parse_rejects_invalid() {
        for tag in &["", "e", "toolonglanguagetag", "en_", "en-US-!", "123"] {
            assert!(Locale::parse(tag).is_err(), "{tag} should be rejected");
        }
    }

    #[test]
    fn normalization() {
        assert_eq!(Locale::parse("EN-us").unwrap().as_str(), "en-US");
        assert_eq!(Locale::parse("ZH-hans").unwrap().as_str(), "zh-Hans");
    }

    #[test]
    fn matches_language() {
        let locale = Locale::parse("en-US").unwrap();
        assert!(locale.matches_language("en"));
        assert!(locale.matches_language("EN"));
        assert!(!locale.matches_language("es"));
    }
}
