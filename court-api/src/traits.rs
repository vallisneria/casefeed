use crate::CaseProvider;
use url::Url;

pub trait HasUrl {
    fn url(&self, provider: &CaseProvider) -> Url;
}
