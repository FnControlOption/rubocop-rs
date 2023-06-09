use rubocop::*;

const COP: &dyn cop::Base = &cop::style::RedundantFilterChain;

config!(
    "
    AllCops:
      ActiveSupportExtensionsEnabled: false
    "
);

const METHODS: &[&str] = &["select", "filter", "find_all"];

#[test]
fn test_method_followed_by_any() {
    for method in METHODS {
        expect_offense! {
            config = config();
            cop = COP;
            replace = { "method" => method };
            source =
                "
                arr.%{method} { |x| x > 1 }.any?
                    ^{method}^^^^^^^^^^^^^^^^^^^ Use `any?` instead of `%{method}.any?`.
                ";
            correction =
                "
                arr.any? { |x| x > 1 }
                ";
        }
    }
}

#[test]
fn test_method_followed_by_empty() {
    for method in METHODS {
        expect_offense! {
            config = config();
            cop = COP;
            replace = { "method" => method };
            source =
                "
                arr.%{method} { |x| x > 1 }.empty?
                    ^{method}^^^^^^^^^^^^^^^^^^^^^ Use `none?` instead of `%{method}.empty?`.
                ";
            correction =
                "
                arr.none? { |x| x > 1 }
                ";
        }
    }
}

#[test]
fn test_method_followed_by_none() {
    for method in METHODS {
        expect_offense! {
            config = config();
            cop = COP;
            replace = { "method" => method };
            source =
                "
                arr.%{method} { |x| x > 1 }.none?
                    ^{method}^^^^^^^^^^^^^^^^^^^^ Use `none?` instead of `%{method}.none?`.
                ";
            correction =
                "
                arr.none? { |x| x > 1 }
                ";
        }
    }
}

#[test]
fn test_method_with_block_pass_followed_by_none() {
    for method in METHODS {
        expect_offense! {
            config = config();
            cop = COP;
            replace = { "method" => method };
            source =
                "
                arr.%{method}(&:odd?).none?
                    ^{method}^^^^^^^^^^^^^^ Use `none?` instead of `%{method}.none?`.
                ";
            correction =
                "
                arr.none?(&:odd?)
                ";
        }
    }
}

#[test]
fn test_method_followed_by_many() {
    for method in METHODS {
        expect_no_offenses! {
            config = config();
            cop = COP;
            replace = { "method" => method };
            source =
                "
                arr.%{method} { |x| x > 1 }.many?
                ";
        }
    }
}

#[test]
fn test_method_without_a_block_followed_by_any() {
    for method in METHODS {
        expect_no_offenses! {
            config = config();
            cop = COP;
            replace = { "method" => method };
            source =
                "
                relation.%{method}(:name).any?
                foo.%{method}.any?
                ";
        }
    }
}

#[test]
fn test_method_followed_by_any_with_arguments() {
    for method in METHODS {
        expect_no_offenses! {
            config = config();
            cop = COP;
            replace = { "method" => method };
            source =
                "
                arr.%{method}(&:odd?).any?(Integer)
                arr.%{method}(&:odd?).any? { |x| x > 10 }
                ";
        }
    }
}

#[test]
fn test_any() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            arr.any? { |x| x > 1 }
            ";
    }
}

mod active_support_extensions_enabled {
    use super::*;

    config!(
        "
        AllCops:
          ActiveSupportExtensionsEnabled: true
        "
    );

    #[test]
    fn test_select_followed_by_many() {
        expect_offense! {
            config = config();
            cop = COP;
            source =
                "
                arr.select { |x| x > 1 }.many?
                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ Use `many?` instead of `select.many?`.
                ";
            correction =
                "
                arr.many? { |x| x > 1 }
                ";
        }
    }
}
