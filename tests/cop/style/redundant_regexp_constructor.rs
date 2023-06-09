use rubocop::*;

const COP: &dyn cop::Base = &cop::style::RedundantRegexpConstructor;

#[test]
fn test_wrapping_regexp_with_regexp_new() {
    expect_offense! {
        cop = COP;
        source =
            "
            Regexp.new(/regexp/)
            ^^^^^^^^^^^^^^^^^^^^ Remove the redundant `Regexp.new`.
            ";
        correction =
            "
            /regexp/
            ";
    }
}

#[test]
fn test_wrapping_regexp_with_cbase_regexp_new() {
    expect_offense! {
        cop = COP;
        source =
            "
            ::Regexp.new(/regexp/)
            ^^^^^^^^^^^^^^^^^^^^^^ Remove the redundant `Regexp.new`.
            ";
        correction =
            "
            /regexp/
            ";
    }
}

#[test]
fn test_wrapping_regexp_i_with_regexp_new() {
    expect_offense! {
        cop = COP;
        source =
            "
            Regexp.new(/regexp/i)
            ^^^^^^^^^^^^^^^^^^^^^ Remove the redundant `Regexp.new`.
            ";
        correction =
            "
            /regexp/i
            ";
    }
}

#[test]
fn test_wrapping_a_regexp_z_io_with_regexp_new() {
    expect_offense! {
        cop = COP;
        source =
            r#"
            Regexp.new(/\A#{regexp}\z/io)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Remove the redundant `Regexp.new`.
            "#;
        correction =
            r#"
            /\A#{regexp}\z/io
            "#;
    }
}

#[test]
fn test_wrapping_regexp_with_regexp_compile() {
    expect_offense! {
        cop = COP;
        source =
            "
            Regexp.compile(/regexp/)
            ^^^^^^^^^^^^^^^^^^^^^^^^ Remove the redundant `Regexp.compile`.
            ";
        correction =
            "
            /regexp/
            ";
    }
}

#[test]
fn test_wrapping_string_literal_with_regexp_new() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Regexp.new('regexp')
            ";
    }
}

#[test]
fn test_wrapping_string_literal_with_regexp_new_with_regopt_argument() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Regexp.new('regexp', Regexp::IGNORECASE)
            ";
    }
}

#[test]
fn test_wrapping_string_literal_with_regexp_new_with_piped_regopt_argument() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Regexp.new('regexp', Regexp::IGNORECASE | Regexp::IGNORECASE)
            ";
    }
}

#[test]
fn test_wrapping_string_literal_with_regexp_compile() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Regexp.compile('regexp')
            ";
    }
}

#[test]
fn test_regexp_literal() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            /regexp/
            ";
    }
}
