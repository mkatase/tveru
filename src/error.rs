// tveru/src/error.rs

use std::fmt;

#[derive(Debug)]
pub enum TveruError {
    Io(std::io::Error),
    WebDriver(thirtyfour::error::WebDriverError),
    Display(String),
    Launch(String),
}

impl fmt::Display for TveruError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TveruError::Io(err) => write!(f, "🚨 I/O Error: {}", err),
            TveruError::WebDriver(err) => write!(f, "🌐 Browser Error: {}", err),
            TveruError::Display(msg) => write!(f, "🎨 Display Error: {}", msg),
            TveruError::Launch(msg) => write!(f, "🎬 Player Error: {}", msg),
        }
    }
}

/// thirtyfourのエラーをTveruError::WebDriverに変換
impl From<thirtyfour::error::WebDriverError> for TveruError {
    fn from(err: thirtyfour::error::WebDriverError) -> Self {
        TveruError::WebDriver(err)
    }
}

/// std::io::ErrorをTveruError::Ioに変換
impl From<std::io::Error> for TveruError {
    fn from(err: std::io::Error) -> Self {
        TveruError::Io(err)
    }
}

/// 共通のResult型を定義(他モジュールでの記述が楽になる)
pub type Result<T> = std::result::Result<T, TveruError>;
