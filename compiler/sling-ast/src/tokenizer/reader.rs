use sling_cache::{Decode, Encode};

#[derive(Decode, Encode, Hash)]
pub struct Reader;

impl Reader {
    pub fn new() -> Self {
        Self
    }
}
