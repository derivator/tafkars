use serde::{Deserialize, Serialize};
/// JSON list response.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListingData<T> {
    /// Modhash
    pub modhash: Option<String>,
    /// The number of children in the listing.
    pub dist: Option<i32>,
    /// The fullname of the listing that follows after this page.
    pub after: Option<String>,
    /// The fullname of the listing that follows before this page.
    pub before: Option<String>,
    /// A list of `things` that this Listing wraps.
    pub children: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename = "Listing")]
pub struct Listing<T> {
    pub data: ListingData<T>,
}

impl<T> Listing<T> {
    // TODO: require one of after/before and modhash/dist
    pub fn new(items: Vec<T>) -> Self {
        Self {
            data: ListingData {
                modhash: None,
                dist: None,
                after: None,
                before: None,
                children: items,
            },
        }
    }
    pub fn push(&mut self, item: T) {
        self.data.children.push(item);
    }
}
