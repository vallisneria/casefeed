mod case_provider;
mod constitutional_court;
mod court;
mod error;
mod traits;
mod util;

pub use case_provider::CaseProvider;
pub use constitutional_court::*;
pub use court::*;
pub use error::CourtApiError;
pub use traits::HasUrl;

pub const USER_AGENT: &'static str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Safari/605.1.15";
