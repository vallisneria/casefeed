pub mod traits;

use db::constitutional_court::ConstitutionalPrecedentDB;
use db::court::CourtPrecedentDB;
use rss::{GuidBuilder, Item, ItemBuilder};
use traits::{ConstitutionalPrecedentDBWrapper, CourtPrecedentDBWrapper};

pub fn generate_rss<T>(items: Vec<T>)
where
    T: Into<Item>,
{
}
