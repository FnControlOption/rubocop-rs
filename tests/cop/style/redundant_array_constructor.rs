use rubocop::*;

const COP: &dyn cop::Base = &cop::style::RedundantArrayConstructor;

#[test]
fn test_empty_array_literal_argument_for_array_new() {
    expect_offense! {
        cop = COP;
        source =
            "
            Array.new([])
            ^^^^^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            []
            ";
    }
}

#[test]
fn test_empty_array_literal_argument_for_cbase_array_new() {
    expect_offense! {
        cop = COP;
        source =
            "
            ::Array.new([])
            ^^^^^^^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            []
            ";
    }
}

#[test]
fn test_empty_array_literal_argument_for_array_index() {
    expect_offense! {
        cop = COP;
        source =
            "
            Array[]
            ^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            []
            ";
    }
}

#[test]
fn test_empty_array_literal_argument_for_cbase_array_index() {
    expect_offense! {
        cop = COP;
        source =
            "
            ::Array[]
            ^^^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            []
            ";
    }
}

#[test]
fn test_empty_array_literal_argument_for_array_constructor() {
    expect_offense! {
        cop = COP;
        source =
            "
            Array([])
            ^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            []
            ";
    }
}

#[test]
fn test_array_literal_with_some_elements_as_argument_for_array_new() {
    expect_offense! {
        cop = COP;
        source =
            "
            Array.new(['foo', 'bar', 'baz'])
            ^^^^^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            ['foo', 'bar', 'baz']
            ";
    }
}

#[test]
fn test_array_literal_with_some_elements_as_argument_for_array_index() {
    expect_offense! {
        cop = COP;
        source =
            "
            Array['foo', 'bar', 'baz']
            ^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            ['foo', 'bar', 'baz']
            ";
    }
}

#[test]
fn test_array_literal_with_some_elements_as_argument_for_array_constructor() {
    expect_offense! {
        cop = COP;
        source =
            "
            Array(['foo', 'bar', 'baz'])
            ^^^^^ Remove the redundant `Array` constructor.
            ";
        correction =
            "
            ['foo', 'bar', 'baz']
            ";
    }
}

#[test]
fn test_array_literal() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            []
            ";
    }
}

#[test]
fn test_single_argument_for_array_new() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Array.new(array)
            ";
    }
}

#[test]
fn test_single_argument_for_array_constructor() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Array(array)
            ";
    }
}

#[test]
fn test_two_arguments_for_array_new() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Array.new(3, 'foo')
            ";
    }
}

#[test]
fn test_block_argument_for_array_new() {
    expect_no_offenses! {
        cop = COP;
        source =
            "
            Array.new(3) { 'foo' }
            ";
    }
}
