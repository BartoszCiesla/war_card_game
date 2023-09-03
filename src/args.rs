use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter};

use crate::seed::SeedRange;

#[derive(Default, ValueEnum, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum Output {
    OneLine,
    Quiet,
    #[default]
    Normal,
    Verbose,
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneLine => write!(f, "OneLine"),
            Self::Quiet => write!(f, "Quiet"),
            Self::Normal => write!(f, "Normal"),
            Self::Verbose => write!(f, "Verbose"),
        }
    }
}

#[derive(Parser)]
#[command(version, author, about = "Application simulates war card game", long_about = None)]
pub(crate) struct Args {
    /// Seed for deck shuffling
    #[arg(short, long, group = "seed_input")]
    seed: Option<u64>,

    /// Range (start-end) of seeds for deck shuffling
    #[arg(long, short = 'r', value_parser = clap::value_parser!(SeedRange), group = "seed_input")]
    range: Option<SeedRange>,

    /// Output level
    #[arg(long, short = 'o', default_value = "normal", group = "output_mode")]
    output: Output,

    /// Verbose mode
    #[arg(long, short = 'v', default_value = "false", group = "output_mode")]
    verbose: bool,

    /// Quiet mode
    #[arg(long, short = 'q', default_value = "false", group = "output_mode")]
    quiet: bool,

    /// One line output mode
    #[arg(long, short = '1', default_value = "false", group = "output_mode")]
    one_line: bool,
}

impl Args {
    pub(crate) fn get_seed(&self) -> Option<SeedRange> {
        if let Some(seed) = self.seed {
            Some(SeedRange::new(seed, seed))
        } else {
            self.range.as_ref().cloned()
        }
    }

    fn is_seed_range(&self) -> bool {
        self.seed.is_none() && self.range.is_some()
    }

    fn is_output_default(&self) -> bool {
        self.output == Output::Normal && !self.one_line && !self.quiet && !self.verbose
    }

    pub(crate) fn get_output(&self) -> Output {
        if self.one_line || (self.is_output_default() && self.is_seed_range()) {
            Output::OneLine
        } else if self.quiet {
            Output::Quiet
        } else if self.verbose {
            Output::Verbose
        } else {
            self.output.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_output() {
        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME")];
        assert_eq!(
            Args::try_parse_from(arg_vec).unwrap().get_output(),
            Output::Normal
        );

        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME"), "-s0"];
        assert_eq!(
            Args::try_parse_from(arg_vec).unwrap().get_output(),
            Output::Normal
        );

        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME"), "-r0-1"];
        assert_eq!(
            Args::try_parse_from(arg_vec).unwrap().get_output(),
            Output::OneLine
        );
    }

    #[test]
    fn test_seed_range() {
        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME")];
        assert!(Args::try_parse_from(arg_vec).unwrap().get_seed().is_none());

        let seed: u64 = 12345;
        let seed_str = format!("-s{}", seed);
        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME"), seed_str.as_str()];
        let seed_range = Args::try_parse_from(arg_vec).unwrap().get_seed().unwrap();
        assert_eq!(seed_range.len(), 1);
        assert_eq!(seed_range.start(), seed);
        assert_eq!(seed_range.end(), seed);

        let seed_start: u64 = 876555;
        let seed_end: u64 = 99879876;
        let seed_str = format!("-r{}-{}", seed_start, seed_end);
        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME"), seed_str.as_str()];
        let seed_range = Args::try_parse_from(arg_vec).unwrap().get_seed().unwrap();
        assert_eq!(seed_range.len(), seed_end - seed_start + 1);
        assert_eq!(seed_range.start(), seed_start);
        assert_eq!(seed_range.end(), seed_end);
    }

    #[test]
    fn test_output_mode() {
        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME"), "-q"];
        assert_eq!(
            Args::try_parse_from(arg_vec).unwrap().get_output(),
            Output::Quiet
        );

        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME"), "-1"];
        assert_eq!(
            Args::try_parse_from(arg_vec).unwrap().get_output(),
            Output::OneLine
        );

        let arg_vec: Vec<&str> = vec![env!("CARGO_PKG_NAME"), "-v"];
        assert_eq!(
            Args::try_parse_from(arg_vec).unwrap().get_output(),
            Output::Verbose
        );
    }

    #[test]
    fn test_output() {
        assert_eq!(Output::Normal.to_string(), "Normal");
        assert_eq!(Output::OneLine.to_string(), "OneLine");
        assert_eq!(Output::Quiet.to_string(), "Quiet");
        assert_eq!(Output::Verbose.to_string(), "Verbose");
    }
}
