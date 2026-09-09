
/// Simple plural rule selection based on a count.
///
/// This covers the CLDR "cardinal" categories zero/one/other for a
/// pragmatic subset of languages. Full CLDR plural rules (few, many,
/// dual, fractions) are future work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralRule {
    /// Used for zero (e.g. Arabic, Latvian zero forms).
    Zero,
    /// Used for one (English singular, Spanish uno, etc.).
    One,
    /// Used for everything else (English plural, etc.).
    Other,
}

impl PluralRule {
    /// Selects a plural rule for `count` using English-style rules
    /// (0 = other, 1 = one, everything else = other).
    #[must_use]
    pub fn for_count(count: u64) -> Self {
        match count {
            1 => Self::One,
            _ => Self::Other,
        }
    }

    /// Selects a plural rule for a language with explicit zero handling
    /// (e.g. Arabic, Latvian).
    #[must_use]
    pub fn with_zero(count: u64) -> Self {
        match count {
            0 => Self::Zero,
            1 => Self::One,
            _ => Self::Other,
        }
    }

    /// Returns the key suffix for this rule: `"zero"`, `"one"`, `"other"`.
    #[must_use]
    pub fn as_key_suffix(&self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::One => "one",
            Self::Other => "other",
        }
    }

    /// Custom rule selection hook for language-specific plural logic.
    #[must_use]
    pub fn custom<F>(count: u64, rule_fn: F) -> Self
    where
        F: Fn(u64) -> Self,
    {
        rule_fn(count)
    }
}

impl std::fmt::Display for PluralRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_key_suffix())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_rules() {
        assert_eq!(PluralRule::for_count(0), PluralRule::Other);
        assert_eq!(PluralRule::for_count(1), PluralRule::One);
        assert_eq!(PluralRule::for_count(2), PluralRule::Other);
        assert_eq!(PluralRule::for_count(u64::MAX), PluralRule::Other);
    }

    #[test]
    fn zero_aware_rules() {
        assert_eq!(PluralRule::with_zero(0), PluralRule::Zero);
        assert_eq!(PluralRule::with_zero(1), PluralRule::One);
        assert_eq!(PluralRule::with_zero(5), PluralRule::Other);
    }

    #[test]
    fn key_suffix() {
        assert_eq!(PluralRule::One.as_key_suffix(), "one");
        assert_eq!(PluralRule::Other.as_key_suffix(), "other");
        assert_eq!(PluralRule::Zero.as_key_suffix(), "zero");
    }

    #[test]
    fn custom_rules() {
        // Arabic-style: 0=zero, 1=one, 2=two, 3-10=few, 11-99=many, rest=other.
        let arabic = |n: u64| match n {
            0 => PluralRule::Zero,
            1 => PluralRule::One,
            2 => PluralRule::Other, // no Two variant in our enum
            3..=10 => PluralRule::Other,
            _ => PluralRule::Other,
        };
        assert_eq!(arabic(0), PluralRule::Zero);
        assert_eq!(arabic(1), PluralRule::One);
    }

    #[test]
    fn display() {
        assert_eq!(PluralRule::One.to_string(), "one");
    }
}
