pub mod errors;
pub mod hasher;
pub mod md;
pub mod file;

use color_eyre::eyre::Result;

use lazy_static::lazy_static;
use md::reporting::md_file;
use regex::Regex;
use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

#[derive(Parser, Debug)]
#[command(name = "Context CLI")]
#[command(author = "Ken Snyder<ken@ken.net>")]
#[command(version,long_about= None)]
struct Cli {
    #[arg(short)]
    /// show more verbose output
    v: bool,

    #[arg(long)]
    /// force output to JSON format
    json: bool,

    /// items which you want context on
    targets: Vec<String>
}


/// **Fingerprint** enum
/// 
/// Provides a list of all _identifiable_ targets which this CLI can
/// give context on.
#[derive(Serialize, Deserialize, Clone)]
pub enum Fingerprint {
    MarkdownFile,
    HtmlFile,

    /// a target string which matches none of the regex patterns currently
    /// in this library
    Unknown
}

struct Matcher {
    re: Regex,
    kind: Fingerprint
}

pub struct Target {
    pub user_input: String,
    pub kind: Fingerprint
}

lazy_static! {
    static ref MATCHERS: [Matcher; 2] = [
        Matcher { kind: Fingerprint::MarkdownFile, re:  Regex::new(r"\w\.md$").unwrap() },
        Matcher { kind: Fingerprint::HtmlFile, re:  Regex::new(r"\w\.htm(l){0,1}$").unwrap() }
    ];
}

fn html_file(target: &Target) -> Result<Value> {
    eprintln!("- '{}' is being processed as a local HTML file", target.user_input);

    Ok(json!("{}"))
}



/// Takes a list of all fingerprints received from user and processes
/// the _known_ fingerprints.
fn process_known_fingerprints(targets: &Vec<Target>) -> Result<Value> {
    let results: Vec<Result<Value>> = targets.iter().map(|t| {
        let result = match t.kind {
            Fingerprint::HtmlFile => html_file(t),
            Fingerprint::MarkdownFile => md_file(t),
            Fingerprint::Unknown => Ok(json!({})),
        };

        result
    }).collect();

    let _errors = results
        .iter() 
        .filter(|i| i.is_err());

        // .collect();
    let outcomes: Vec<Value> = results
        .into_iter()
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect();

    Ok(json!(outcomes))
}

/**
 * Sends warning messages to stderr for any unknown fingerprints and returns
 * a boolean value indicating if _any_ of the fingerprints were unknown.
 * 
 * Note: a `true` return means there **were** unknown fingerprints
 */
fn warn_about_unknown_fingerprints(targets: &Vec<Target>) -> bool {
    let mut found = false;
    targets.iter().for_each(|i| {
        if let Fingerprint::Unknown = i.kind {
            eprintln!("- '{0}' was not recognized and will be ignored!", i.user_input);
            found = true;
        }
    });

    found
}

/// Tests whether the input string matches a known matcher pattern which will
/// contextualize what a given "target" is. At most one match will be found as
/// match conditions are evaluated lazily until a match is found.
/// 
/// For debugging purposes, you may want to try `matches(input)` function instead
/// as it will return ALL matches.
fn fingerprint(input: &str) -> Target {
    let found = MATCHERS.iter().find(|m| {
        if m.re.is_match(input) {
            true
        } else {
            false
        }
    });

    match found {
        Some(m) => Target { kind: m.kind.clone(), user_input: input.to_string() },
        None => Target { kind: Fingerprint::Unknown, user_input: input.to_string()}
    }
}


fn main() {

    let args = Cli::parse();
    let multiple_targets = args.targets.len() > 1;

    if multiple_targets {
        eprintln!("Context CLI: processing {:?} targets", args.targets.len());
    } else {
        eprintln!("Context CLI");
    }
    eprintln!("-----------------------------------");
    eprintln!("");
    println!("targets are: {:?}", args.targets);

    let fingerprints: Vec<Target> = args.targets.iter().map(|i| fingerprint(&i)).collect();
    let _results = process_known_fingerprints(&fingerprints);
    let _had_unknown = warn_about_unknown_fingerprints(&fingerprints);

    
}
