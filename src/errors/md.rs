use serde_json::Error as JsonError;
use serde_json::Value;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("Invalid JSON value to be passed in as proxy for a Frontmatter value")]
    InvalidFrontmatter(#[from] JsonError),

    #[error("Frontmatter restricts a few hash based property names from being used in the Frontmatter key/value hash; that includes the property {0} but there was an attempt to set it to {1}")]
    PropertyCanNotBeSet(String, Value),

    #[error("While trying to set the {0} property on Frontmatter we ran into a type error; this property was expected to be a {1}.")]
    PropertyIsWrongType(String, String),


}
