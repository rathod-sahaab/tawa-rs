// trybuild tests for ImplPolylineConst compile-time errors

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/invalid_nan.rs");
    t.compile_fail("tests/ui/invalid_dup.rs");
}
