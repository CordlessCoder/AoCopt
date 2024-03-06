use std::{borrow::Cow, collections::HashMap, fmt::Debug, path::PathBuf, time::Duration};
mod deserialize;

use clap::{Parser, ValueEnum};
use regex::Regex;
use serde::Deserialize;
use serde_with::{formats::Flexible, serde_as, DurationSeconds};

pub use deserialize::DeserFromStr;

#[derive(Parser, Debug, Clone)]
/// My comfy AoC benchmarking and testing harness. Enjoy!
pub struct Arguments {
    /// The year to run tests for.
    #[arg(short, long, default_value_t = 2023, value_parser = clap::value_parser!(u16).range(2015..))]
    pub year: u16,
    /// The day to run tests for.
    #[arg(short, value_parser = clap::value_parser!(u8).range(1..=31), default_value_t = 1)]
    pub day: u8,
    /// The part(s) to run tests for.
    /// Runs all parts if not specified.
    #[arg(short, value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: Option<u8>,
    /// The languages to run the tests for.
    /// Runs all languages if not specified.
    #[arg(short, long)]
    pub lang: Vec<String>,
    /// The AOC session token to use for downloading inputs
    #[arg(env, long)]
    pub aoc_token: Option<String>,
    /// The solutions to attempt running.
    /// Leave empty to run all defined solutions.
    #[arg(long, short)]
    pub bench: Vec<String>,
    /// Run the clean hook of the solutions.
    #[arg(long, short)]
    pub clean: bool,
    /// The config file to use.
    #[arg(long, default_value = "config.toml")]
    pub config: PathBuf,
    /// Print the solutions that would be ran, without actually running them.
    #[arg(long)]
    pub dry: bool,
    /// Don't emit color in the output tables
    #[arg(long)]
    pub nocolor: bool,
    /// The style used for the results table
    #[arg(long, env = "TABLE_STYLE", default_value = "sharp")]
    pub table: TableStyle,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default)]
pub enum TableStyle {
    Markdown,
    Modern,
    ModernRounded,
    PSql,
    ReRestructuredText,
    Dots,
    Ascii,
    AsciiRounded,
    Blank,
    TwoLine,
    #[default]
    Sharp,
    Rounded,
}

fn get_true() -> bool {
    true
}

fn default_input_path() -> PathBuf {
    PathBuf::from("inputs")
}
fn default_duration() -> Duration {
    Duration::from_secs(60 * 60)
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "get_true")]
    pub download_inputs: bool,
    #[serde(
        default = "default_input_path",
        alias = "inputs",
        alias = "inputs_path"
    )]
    pub input_path: PathBuf,
    #[serde_as(as = "DurationSeconds<String, Flexible>")]
    #[serde(default = "default_duration", alias = "request_timeout")]
    pub req_timeout: Duration,
    #[serde_as(as = "DurationSeconds<String, Flexible>")]
    #[serde(default = "default_duration", alias = "cmd_timeout")]
    pub command_timeout: Duration,
    #[serde(flatten)]
    pub solutions: HashMap<
        DeserFromStr<u16>,
        HashMap<DeserFromStr<u8>, HashMap<DeserFromStr<u8>, Vec<Solution>>>,
    >,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Solution {
    #[serde(alias = "lang")]
    pub language: String,
    pub name: String,
    #[serde(alias = "desc")]
    pub description: Option<String>,

    pub build: Option<String>,
    #[serde(alias = "pre_exec", alias = "pre")]
    pub pre_hook: Option<String>,

    #[serde(alias = "execute")]
    pub exec: String,

    #[serde(alias = "post_exec", alias = "post")]
    pub post_hook: Option<String>,
    #[serde(alias = "clean")]
    pub clean_hook: Option<String>,
    #[serde(alias = "custom_shell")]
    pub shell: Option<Vec<Cow<'static, str>>>,
    #[serde(alias = "out", default)]
    pub output: Output,
    #[serde(
        alias = "time_pattern",
        with = "serde_regex",
        default = "default_time_regex"
    )]
    pub time_regex: Regex,
    #[serde(
        alias = "result_pattern",
        with = "serde_regex",
        default = "default_result_regex"
    )]
    pub result_regex: Regex,
    #[serde(
        alias = "instrument",
        alias = "no_builtin_timing",
        alias = "external_timing",
        alias = "time_ext",
        default
    )]
    pub time_externally: bool,
    #[serde(alias = "cwd", alias = "pwd", alias = "directory", alias = "dir")]
    pub path: Option<PathBuf>,
}
fn default_time_regex() -> Regex {
    Regex::new(r"(?m)^\d+[^\d]+(\d+)").expect("Default regex is invalid!")
}
fn default_result_regex() -> Regex {
    Regex::new(r"(?m)^(\d+)[^\d]+\d+").expect("Default regex is invalid!")
}
#[derive(Deserialize, Debug, Clone, Default, Copy, PartialEq)]
pub enum Output {
    #[serde(alias = "stdout", alias = "STDOUT")]
    #[default]
    Stdout,
    #[serde(alias = "stderr", alias = "STDERR")]
    Stderr,
}

// #[derive(Deserialize, Debug, Clone, Copy, Default)]
// pub enum Timing {
//     Internal,
//     #[default]
//     External,
// }
