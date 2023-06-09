use rubocop::*;

const COP: &dyn cop::Base = &cop::style::ExactRegexpMatch;

#[test]
fn test_match_operator() {
    expect_offense! {
        cop = COP;
        source =
            r#"
            string =~ /\Astring\z/
            ^^^^^^^^^^^^^^^^^^^^^^ Use `string == 'string'`.
            "#;
        correction =
            "
            string == 'string'
            ";
    }
}

#[test]
fn test_case_equality_operator() {
    expect_offense! {
        cop = COP;
        source =
            r#"
            string === /\Astring\z/
            ^^^^^^^^^^^^^^^^^^^^^^^ Use `string == 'string'`.
            "#;
        correction =
            "
            string == 'string'
            ";
    }
}

#[test]
fn test_match() {
    expect_offense! {
        cop = COP;
        source =
            r#"
            string.match(/\Astring\z/)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^ Use `string == 'string'`.
            "#;
        correction =
            "
            string == 'string'
            ";
    }
}

#[test]
fn test_match_p() {
    expect_offense! {
        cop = COP;
        source =
            r#"
            string.match?(/\Astring\z/)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ Use `string == 'string'`.
            "#;
        correction =
            "
            string == 'string'
            ";
    }
}

#[test]
fn test_not_match_operator() {
    expect_offense! {
        cop = COP;
        source =
            r#"
            string !~ /\Astring\z/
            ^^^^^^^^^^^^^^^^^^^^^^ Use `string != 'string'`.
            "#;
        correction =
            "
            string != 'string'
            ";
    }
}

#[test]
fn _focus_test_string_interpolation() {
    expect_no_offenses! {
        cop = COP;
        source =
            r#"
            string =~ /\Astring#{interpolation}\z/
            "#;
    }
}

#[test]
fn test_literal_with_qualifier() {
    expect_no_offenses! {
        cop = COP;
        source =
            r#"
            string === /\A0+\z/
            "#;
    }
}

#[test]
fn test_any_pattern() {
    expect_no_offenses! {
        cop = COP;
        source =
            r#"
            string =~ /\Astring.*\z/
            "#;
    }
}

#[test]
fn test_multiline_matches() {
    expect_no_offenses! {
        cop = COP;
        source =
            r#"
            string =~ /^string$/
            "#;
    }
}

#[test]
fn test_regexp_opt() {
    expect_no_offenses! {
        cop = COP;
        source =
            r#"
            string =~ /\Astring\z/i
            "#;
    }
}
