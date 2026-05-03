#[test]
pub fn test_macro_expand() {
    macrotest::expand("tests/expands/*.rs");
}
