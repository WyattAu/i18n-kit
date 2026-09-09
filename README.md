# i18n-kit

Runtime i18n for Rust — BCP 47 locales, dot-notation catalogs, fallback chains, `{placeholder}` interpolation, and simple plural rules. Zero-dep core (serde optional).

## Quick start

```rust
use i18n_kit::{Catalog, Locale};

let mut catalog = Catalog::new();
catalog.insert("en", "greeting", "Hello, {name}!");
catalog.insert("es", "greeting", "¡Hola, {name}!");

let translator = catalog.into_translator("en");
assert_eq!(
    translator.translate("es", "greeting", &[("name", "Mundo")]),
    "¡Hola, Mundo!"
);
```

## Features

- BCP 47 locale parsing and normalization (language/script/region casing)
- Dot-notation keys (`"nav.home"`, `"errors.404"`)
- Fallback chain: exact → language-only → default locale → key itself
- `{placeholder}` interpolation
- Plural rules (zero/one/other, extensible via closure)
- `serde` + `json` features for loading from JSON files

## License

MIT OR Apache-2.0
