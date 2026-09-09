#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Runtime i18n for Rust — BCP 47 locales, dot-notation catalogs,
//! fallback chains, `{placeholder}` interpolation, and simple plural rules.
//!
//! Unlike `fluent` (compile-time checked, full CLDR), `i18n-kit` provides
//! runtime string lookup with a minimal dependency footprint. Load catalogs
//! from JSON/YAML at startup or embed them with `include_str!`.
//!
//! # Quick start
//!
//! ```
//! use i18n_kit::{Catalog, Locale};
//!
//! let mut catalog = Catalog::new();
//! catalog.insert("en", "greeting", "Hello, {name}!");
//! catalog.insert("es", "greeting", "¡Hola, {name}!");
//!
//! let translator = catalog.into_translator("en");
//! assert_eq!(
//!     translator.translate("es", "greeting", &[("name", "Mundo")]),
//!     "¡Hola, Mundo!"
//! );
//! ```

mod catalog;
mod error;
mod locale;
mod plural;
mod translator;

pub use catalog::Catalog;
pub use error::I18nError;
pub use locale::Locale;
pub use plural::PluralRule;
pub use translator::Translator;

/// Interpolation placeholder delimiters.
pub(crate) const PLACEHOLDER_START: char = '{';
pub(crate) const PLACEHOLDER_END: char = '}';
