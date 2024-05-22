use testing::*;

#[test]
fn yet_more_tests() {
    // - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}