// [Gray Matter](https://docs.rs/gray_matter/latest/gray_matter/)

use crate::errors::md::MarkdownError;
use tracing::{debug, instrument};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub enum FrontmatterEngineType {
    YAML,
    JSON,
    TOML,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FmHashValues {
    /// A hash value representing the frontmatter immediately after it is
    /// extracted from the `MarkdownContentRaw`
    pub extracted: Option<u64>,
    /// A hash value representing the frontmatter after the default values
    /// hook has been applied to the page content
    pub defaults_applied: Option<u64>,
    /// A hash value representing the frontmatter after both the _defaults_
    /// and _overrides_ hooks have been applied to the page content.    
    pub overrides_applied: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    default
)]
pub struct Frontmatter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_auth: Option<bool>,
    /// Other properties who's type are not known until run time
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl Default for Frontmatter {
    #[instrument]
    fn default() -> Frontmatter {
        Frontmatter::new(None).unwrap()
    }
}

impl Display for Frontmatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap();
        write!(f, "{}", json)
    }
}

impl Into<String> for Frontmatter {
    #[instrument]
    fn into(self) -> String {
        json!(self).to_string()
    }
}

impl TryFrom<String> for Frontmatter {
    type Error = MarkdownError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let matter = Matter::<YAML>::new();
        let fm = Frontmatter::new(
            matter.parse(&value).data.unwrap().deserialize().unwrap()
        )?;
        Ok(fm)
    }
}

impl TryFrom<&str> for Frontmatter {
    type Error = MarkdownError;

    fn try_from(value: &str) -> Result<Self, MarkdownError> {
        Frontmatter::try_from(value.to_string())
    }
}

impl Frontmatter {
    #[instrument]
    pub fn new(json: Option<Value>) -> Result<Self, MarkdownError> {
        if let Some(json) = json {
            let fm: Frontmatter = serde_json::from_value(json.clone())?;

            debug!(
                "New Frontmatter from JSON:\n{}\n\nis translated to {:?}",
                &json, &fm
            );

            Ok(fm)
        } else {
            Ok(Frontmatter {
                title: None,
                description: None,
                aliases: None,
                tags: None,
                subject: None,
                category: None,
                name: None,
                excerpt: None,
                image: None,
                icon: None,
                layout: None,
                requires_auth: None,
                other: HashMap::new(),
            })
        }
    }

}

#[cfg(test)]
mod tests {
    use tracing::{info, Level};
    use tracing_subscriber;

    const SIMPLE_MD: &str = r#"---
title: testing
foo: "bar"
bar: true
baz: 42
---
# Hello World\n
this is a test 
"#;

    fn trace(lvl: Level) {
        let collector = tracing_subscriber::fmt()
            // filter spans/events with level TRACE or higher.
            .with_max_level(lvl)
            .pretty()
            // build but do not install the subscriber.
            .finish();

        tracing::subscriber::set_global_default(collector).unwrap();

        info!("Tracing for tests enabled");
    }


}
