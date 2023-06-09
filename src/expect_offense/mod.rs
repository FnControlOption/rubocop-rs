mod annotated_source;
use annotated_source::AnnotatedSource;

use std::borrow::Cow;

use lib_ruby_parser::{Parser, ParserOptions};

use crate::cop::{self, Offense};
use crate::source::{Corrector, DecodedInput};
use crate::{Commissioner, Config};

pub fn normalize_source(source: &str) -> Cow<str> {
    let lines = source.lines().collect::<Vec<_>>();
    match lines.as_slice() {
        [first, lines @ .., last] if first.is_empty() && last.trim().is_empty() => {
            let lines = lines.iter().map(|s| &s[last.len()..]).collect::<Vec<_>>();
            Cow::from(lines.join("\n"))
        }
        _ => Cow::from(source),
    }
}

fn format_source(source: &str, replacements: &[(&str, &str)]) -> String {
    let mut source = source.to_string();
    for (keyword, value) in replacements {
        source = source
            .replace(&format!("%{{{keyword}}}"), value)
            .replace(&format!("^{{{keyword}}}"), &"^".repeat(value.len()))
            .replace(&format!("_{{{keyword}}}"), &" ".repeat(value.len()));
    }
    source
}

pub fn expect_offense(
    config: Option<&serde_yaml::Mapping>,
    cop: &dyn cop::Base,
    source: &str,
    replacements: &[(&str, &str)],
) -> (DecodedInput, Corrector) {
    let (expected_annotations, source) = parse_annotations(source, replacements);

    let (input, offenses, corrector) = investigate(config, cop, source.as_bytes());

    let actual_annotations = expected_annotations.with_offense_annotations(&offenses, &input);
    assert_eq!(actual_annotations, expected_annotations);

    (input, corrector)
}

pub fn expect_correction(input: DecodedInput, corrector: Corrector, correction: &str) {
    let correction = correction.as_bytes();
    let source = input.as_shared_bytes();

    if correction == source {
        panic!("Use `expect_no_corrections` if the code will not change");
    }

    let new_source = corrector.process(source);

    if new_source == source {
        panic!("Expected correction but no corrections were made");
    }

    let new_source = String::from_utf8_lossy(&new_source);
    let correction = String::from_utf8_lossy(correction);
    assert_eq!(new_source, correction);
}

pub fn expect_no_offenses(
    config: Option<&serde_yaml::Mapping>,
    cop: &dyn cop::Base,
    source: &str,
    replacements: &[(&str, &str)],
) {
    let source = format_source(source, replacements);
    let (input, offenses, corrector) = investigate(config, cop, source.as_bytes());

    let expected_annotations = AnnotatedSource::parse(&source);
    let actual_annotations = expected_annotations.with_offense_annotations(&offenses, &input);
    assert_eq!(actual_annotations, expected_annotations);

    assert!(corrector.is_empty());
}

fn parse_annotations(source: &str, replacements: &[(&str, &str)]) -> (AnnotatedSource, String) {
    let source = format_source(source, replacements);
    let annotations = AnnotatedSource::parse(&source);
    let plain_source = annotations.plain_source();
    if plain_source != source {
        return (annotations, plain_source);
    }

    panic!("Use `expect_no_offenses` to assert that no offenses are found");
}

fn investigate(
    config: Option<&serde_yaml::Mapping>,
    cop: &dyn cop::Base,
    source: &[u8],
) -> (DecodedInput, Vec<Offense>, Corrector) {
    let mut config = match config {
        Some(config) => config.clone(),
        _ => serde_yaml::Mapping::new(),
    };

    let cop_config = config.entry(cop.name().into());
    let cop_config = cop_config.or_insert_with(|| serde_yaml::Mapping::new().into());
    let cop_config = cop_config.as_mapping_mut().unwrap();
    cop_config
        .entry("Enabled".into())
        .or_insert_with(|| true.into());

    let config = config.into();
    let config = Config::new(Some(&config));

    let parser_options = ParserOptions {
        record_tokens: false,
        ..Default::default()
    };
    let parser = Parser::new(source, parser_options);
    Commissioner::investigate(&[cop], config, parser)
}

#[macro_export]
macro_rules! expect_offense {
    (
        cop = $cop:expr;
        source = $source:expr;
        correction = $correction:expr;
    ) => {
        expect_offense! {
            config = None;
            cop = $cop;
            replace = {};
            source = $source;
            correction = $correction;
        }
    };

    (
        config = $config:expr;
        cop = $cop:expr;
        source = $source:expr;
        correction = $correction:expr;
    ) => {
        expect_offense! {
            config = $config;
            cop = $cop;
            replace = {};
            source = $source;
            correction = $correction;
        }
    };

    (
        cop = $cop:expr;
        replace = { $($keyword:expr => $value:expr),* $(,)? };
        source = $source:expr;
        correction = $correction:expr;
    ) => {
        expect_offense! {
            config = None;
            cop = $cop;
            replace = { $($keyword => $value),* };
            source = $source;
            correction = $correction;
        }
    };

    (
        config = $config:expr;
        cop = $cop:expr;
        replace = { $($keyword:expr => $value:expr),* $(,)? };
        source = $source:expr;
        correction = $correction:expr;
    ) => {
        let config = Option::from($config);
        let replacements: &[(&str, &str)] = &[$(($keyword, $value)),*];
        use $crate::expect_offense::*;
        let (input, corrector) = expect_offense(config, $cop, &normalize_source($source), replacements);
        expect_correction(input, corrector, &normalize_source($correction));
    };
}

#[macro_export]
macro_rules! expect_no_offenses {
    (
        cop = $cop:expr;
        source = $source:expr;
    ) => {
        expect_no_offenses! {
            config = None;
            cop = $cop;
            replace = {};
            source = $source;
        }
    };

    (
        config = $config:expr;
        cop = $cop:expr;
        source = $source:expr;
    ) => {
        expect_no_offenses! {
            config = $config;
            cop = $cop;
            replace = {};
            source = $source;
        }
    };

    (
        config = $config:expr;
        cop = $cop:expr;
        replace = { $($keyword:expr => $value:expr),* $(,)? };
        source = $source:expr;
    ) => {
        let config = Option::from($config);
        let replacements: &[(&str, &str)] = &[$(($keyword, $value)),*];
        use $crate::expect_offense::*;
        expect_no_offenses(config, $cop, &normalize_source($source), replacements);
    };
}

#[macro_export]
macro_rules! config {
    ($s:expr) => {
        fn config() -> &'static serde_yaml::Mapping {
            use std::sync::OnceLock;
            static CONFIG: OnceLock<serde_yaml::Mapping> = OnceLock::new();
            CONFIG.get_or_init(|| serde_yaml::from_str($s).unwrap())
        }
    };
}
