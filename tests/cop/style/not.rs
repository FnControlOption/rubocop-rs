use rubocop::*;

const COP: &dyn cop::Base = &cop::style::Not;

#[test]
fn test_not() {
    expect_offense! {
        cop = COP;
        source =
            "
            not test
            ^^^ Use `!` instead of `not`.
            ";
        correction =
            "
            !test
            ";
    }
}

#[test]
fn test_bang() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            ! test
            ";
    }
}

// TODO
