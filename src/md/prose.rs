use serde::{Serialize, Deserialize};

use crate::hasher::hash;

#[derive(Debug,Serialize,Deserialize)]
pub struct Prose {
    pub content: String,
    pub hash: u64
}

impl Prose {
    pub fn new(content: &str) -> Prose {
        Prose {
            hash: hash(content),
            content: content.to_string(),
        }
    }
}

impl From<String> for Prose {
    fn from(content: String) -> Self {
        Self {
            hash: hash(&content),
            content
        }
    }
}

impl From<&str> for Prose {
    fn from(content: &str) -> Self {
        Prose::from(content.to_string())
    }
}
