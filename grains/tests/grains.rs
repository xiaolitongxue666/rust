fn process_square_case(input: u32, expected: u64) {
    assert_eq!(grains::square(input), expected);
}

#[test]
/// 1
fn test_1() {
    process_square_case(1, 1);
}

#[test]
/// 2
fn test_2() {
    process_square_case(2, 2);
}

#[test]
#[ignore]
fn test_3() {
    process_square_case(3, 4);
}

#[test]
/// 4
fn test_4() {
    process_square_case(4, 8);
}

//NEW
#[test]
/// 16
fn test_16() {
    process_square_case(16, 32_768);
}

#[test]
/// 32
fn test_32() {
    process_square_case(32, 2_147_483_648);
}

#[test]
/// 64
fn test_64() {
    process_square_case(64, 9_223_372_036_854_775_808);
}

#[test]
#[should_panic(expected = "Square must be between 1 and 64")]
fn test_square_0_raises_an_exception() {
    grains::square(0);
}

#[test]
#[should_panic(expected = "Square must be between 1 and 64")]
fn test_square_greater_than_64_raises_an_exception() {
    grains::square(65);
}

#[test]
#[ignore]
fn test_returns_the_total_number_of_grains_on_the_board() {
    assert_eq!(grains::total(), 18_446_744_073_709_551_615);
}
