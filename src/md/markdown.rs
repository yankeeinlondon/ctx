use serde::{Serialize, Deserialize};
use tracing::{instrument, info};

use crate::file::{FileMeta, FileWithMeta};
use crate::errors::md::MarkdownError;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use super::{
    prose::Prose, 
    frontmatter::Frontmatter,
};

// [Docs](https://docs.rs/regex/latest/regex/struct.Regex.html)
lazy_static! {
    static ref HAS_FM: Regex = RegexBuilder::new(r"^---\n.?*\n---")
        .multi_line(false)
        .build()
        .unwrap();
    static ref MD_PARTS: Regex = Regex::new(r"^---.*---(.*)").unwrap();
}

/// determines whether content representing the raw
/// text from a Markdown file, contains a frontmatter
/// section or not.
#[instrument]
pub fn has_frontmatter(content: &str) -> bool {
    HAS_FM.is_match(content)
}

/// given a raw content string, this will return a string
/// which ensures that there is NOT any frontmatter content
#[instrument]
pub fn exclude_frontmatter(content: &str) -> String {
    if has_frontmatter(content) {
        let replacement = MD_PARTS.replace(content, "$1").to_string();
        replacement
    } else {
        content.to_string()
    }
}


/// Receives a raw string slice which represents the content of a markdown file
/// and then returns the `Prose` and `Frontmatter` sections as separates.
#[instrument]
pub fn split_fm_from_prose(
    raw_content: &str
) -> Result<(Prose, Option<Frontmatter>), MarkdownError> {

    let frontmatter: Option<Frontmatter>;
    let prose: Prose;

    if has_frontmatter(raw_content) {
        let fm = Frontmatter::try_from(raw_content)?;
        frontmatter = Some(fm);
        prose = Prose::from(exclude_frontmatter(raw_content));
    } else {
        frontmatter = None;
        prose = Prose::from(exclude_frontmatter(raw_content));        
    }

    info!(
        "split markdown content:\n{:?}\n{:?}",
        &frontmatter, &prose
    );
    
    Ok((prose, frontmatter))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownStructure {
    pub h1: Vec<String>,
    pub has_multiple_h1: bool,
    pub h2: Vec<String>,
    pub h3: Vec<String>,
}

/// A markdown document which consists of two major parts:
/// 
/// 1. **Frontmatter** - which is optional structured data defined at the top of a page
/// 2. **Prose** - the real content of a page which represents unstructured or semi-structured content
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownDoc {
    pub has_frontmatter: bool,
    pub fm: Option<Frontmatter>,
    pub prose: Prose,
    pub structure: Option<MarkdownStructure>,
    pub file: Option<FileMeta>
}

/// try to create a `MarkdownDoc` from a string slice which represents the content
/// of a markdown file.
impl TryFrom<&str> for MarkdownDoc {
    type Error = MarkdownError;

    fn try_from(raw_content: &str) -> Result<Self, Self::Error> {
        let (prose, fm) = split_fm_from_prose(raw_content)?;

        Ok(MarkdownDoc {
            has_frontmatter: has_frontmatter(raw_content),
            fm,
            prose,
            structure: None,
            file: None
        })
    }
}

impl TryFrom<FileWithMeta> for MarkdownDoc {
    type Error = MarkdownError;

    fn try_from(value: FileWithMeta) -> Result<Self, Self::Error> {
        let raw = value.content.as_str();
        let (
            prose, 
            fm
        ) = split_fm_from_prose(
            raw
        )?;
        
        Ok(Self {
            has_frontmatter: has_frontmatter(raw),
            fm,
            prose,
            file: Some(value.meta),
            structure: None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{info, Level};
    use tracing_subscriber;

    const PROSE_ONLY: &str = r#"
# Hello World\n

this is a test 
"#;

    const FM_CONTENT: &str = r#"---
title: "testing"
foo: 42
bar: "bar"
baz: "baz"
---

# With Frontmatter

Hello World
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


    #[test]
    fn prose_only_returns_false() {
        assert_eq!(has_frontmatter(PROSE_ONLY), false);
    }

    #[test]
    fn fm_content_returns_true() {
        assert_eq!(has_frontmatter(FM_CONTENT), false);
    }

    #[test]
    fn fm_content_with_extraction_returns_false() {
        assert_eq!(has_frontmatter(&exclude_frontmatter(FM_CONTENT)), false);
    }

    #[test]
    fn fm_content_split_gives_valid_results() {

        let (prose, fm) = split_fm_from_prose(
            FM_CONTENT
        ).unwrap();

        if let Some(fm) = fm {
            let foo = fm.other.get("foo");
            let bar = fm.other.get("bar");
            let baz = fm.other.get("baz");

            assert!(fm.title.is_some());
            assert!(foo.is_some());
            assert!(bar.is_some());
            assert!(baz.is_some());
            if let Some(title) = fm.title {
                assert_eq!(title, "testing");
            };
        } else {
            assert!(fm.is_some()); // fail
        }


    }


}
