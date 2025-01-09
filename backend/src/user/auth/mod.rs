use chrono::Duration;
use tower_cookies::cookie::time::Duration as CookieDuration;

pub mod cookie;
pub mod crypto;
pub mod error;
pub mod session;
pub mod userimpl;

pub const SESSION_COOKIE_NAME: &str = "qesesh";
pub const SESSION_DURATION: Duration = Duration::weeks(1);
pub const SESSION_COOKIE_DURATION: CookieDuration = CookieDuration::weeks(1);
