use color_eyre::eyre::Result;
use serde_json::{Value, json};

use crate::{
    Target, 
    md::markdown::MarkdownDoc, 
    file::{FileMeta, FileWithMeta}
};

pub fn md_file(target: &Target) -> Result<Value> {
    eprintln!("- '{}' is being processed as a local Markdown file", &target.user_input);
    let file = FileMeta::try_from(&target.user_input)?;
    let file = FileWithMeta::try_from(file)?;
    let md = MarkdownDoc::try_from(file)?;

    println!("- markdown {:?}", md);


    // println!("- {0} is {1:?}", target.user_input, md.file?.filename);

    Ok(json!(md))
}
