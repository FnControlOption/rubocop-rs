/*
use rubocop::*;

const COP: &dyn cop::Base = &cop::style::StringLiterals;

mod configured_with_single_quotes_preferred {
    use super::*;

    config!(&format!(
        "
        {COP}:
          EnforcedStyle: single_quotes
        "
    ));

    #[ignore]
    #[test]
    fn test_double_quotes_when_single_quotes_suffice() {
        expect_offense! {
            config = config();
            cop = COP;
            source =
                r#"
                s = "abc"
                    ^^^^^ Prefer single-quoted strings when you don't need string interpolation or special symbols.
                x = "a\\b"
                    ^^^^^^ Prefer single-quoted strings when you don't need string interpolation or special symbols.
                y ="\\b"
                   ^^^^^ Prefer single-quoted strings when you don't need string interpolation or special symbols.
                z = "a\\"
                    ^^^^^ Prefer single-quoted strings when you don't need string interpolation or special symbols.
                t = "{\"[\\\"*\\\"]\""
                    ^^^^^^^^^^^^^^^^^^ Prefer single-quoted strings when you don't need string interpolation or special symbols.
                "#;
            correction =
                r#"
                s = 'abc'
                x = 'a\\b'
                y ='\\b'
                z = 'a\\'
                t = '{"[\"*\"]"'
                "#;
        }
    }
}
*/
