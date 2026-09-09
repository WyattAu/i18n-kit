/// Errors that can occur during i18n operations.
#[derive(Debug, thiserror::Error)]
pub enum I18nError {
    /// Invalid locale tag.
    #[error("invalid locale: {0}")]
    InvalidLocale(String),
}
