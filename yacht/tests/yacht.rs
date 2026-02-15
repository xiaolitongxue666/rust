use yacht::*;

#[test]
fn yacht() {
    assert_eq!(score([5, 5, 5, 5, 5], Category::Yacht), 50);
}

#[test]
#[ignore]
fn not_yacht() {
    assert_eq!(score([1, 3, 3, 2, 5], Category::Yacht), 0);
}

#[test]
#[ignore]
fn ones() {
    assert_eq!(score([1, 1, 1, 3, 5], Category::Ones), 3);
}

#[test]
#[ignore]
fn ones_out_of_order() {
    assert_eq!(score([3, 1, 1, 5, 1], Category::Ones), 3);
}

#[test]
#[ignore]
fn no_ones() {
    assert_eq!(score([4, 3, 6, 5, 5], Category::Ones), 0);
}

#[test]
#[ignore]
fn twos() {
    assert_eq!(score([2, 3, 4, 5, 6], Category::Twos), 2);
}

#[test]
#[ignore]
fn fours() {
    assert_eq!(score([1, 4, 1, 4, 1], Category::Fours), 8);
}

#[test]
#[ignore]
fn yacht_counted_as_threes() {
    assert_eq!(score([3, 3, 3, 3, 3], Category::Threes), 15);
}

#[test]
#[ignore]
fn yacht_of_3s_counted_as_fives() {
    assert_eq!(score([3, 3, 3, 3, 3], Category::Fives), 0);
}

#[test]
#[ignore]
fn fives() {
    assert_eq!(score([1, 5, 3, 5, 3], Category::Fives), 10);
}

#[test]
#[ignore]
fn sixes() {
    assert_eq!(score([2, 3, 4, 5, 6], Category::Sixes), 6);
}

#[test]
#[ignore]
fn full_house_two_small_three_big() {
    assert_eq!(score([2, 2, 4, 4, 4], Category::FullHouse), 16);
}

#[test]
#[ignore]
fn full_house_three_small_two_big() {
    assert_eq!(score([5, 3, 3, 5, 3], Category::FullHouse), 19);
}

#[test]
#[ignore]
fn two_pair_is_not_a_full_house() {
    assert_eq!(score([2, 2, 4, 4, 5], Category::FullHouse), 0);
}

#[test]
#[ignore]
fn four_of_a_kind_is_not_a_full_house() {
    assert_eq!(score([1, 4, 4, 4, 4], Category::FullHouse), 0);
}

#[test]
#[ignore]
fn yacht_is_not_a_full_house() {
    assert_eq!(score([2, 2, 2, 2, 2], Category::FullHouse), 0);
}

#[test]
#[ignore]
fn four_of_a_kind() {
    assert_eq!(score([6, 6, 4, 6, 6], Category::FourOfAKind), 24);
}

#[test]
#[ignore]
fn yacht_can_be_scored_as_four_of_a_kind() {
    assert_eq!(score([3, 3, 3, 3, 3], Category::FourOfAKind), 12);
}

#[test]
#[ignore]
fn full_house_is_not_four_of_a_kind() {
    assert_eq!(score([3, 3, 3, 5, 5], Category::FourOfAKind), 0);
}

#[test]
#[ignore]
fn little_straight() {
    assert_eq!(score([3, 5, 4, 1, 2], Category::LittleStraight), 30);
}

#[test]
#[ignore]
fn little_straight_as_big_straight() {
    assert_eq!(score([1, 2, 3, 4, 5], Category::BigStraight), 0);
}

#[test]
#[ignore]
fn four_in_order_but_not_a_little_straight() {
    assert_eq!(score([1, 1, 2, 3, 4], Category::LittleStraight), 0);
}

#[test]
#[ignore]
fn no_pairs_but_not_a_little_straight() {
    assert_eq!(score([1, 2, 3, 4, 6], Category::LittleStraight), 0);
}

#[test]
#[ignore]
fn minimum_is_1_maximum_is_5_but_not_a_little_straight() {
    assert_eq!(score([1, 1, 3, 4, 5], Category::LittleStraight), 0);
}

#[test]
#[ignore]
fn big_straight() {
    assert_eq!(score([4, 6, 2, 5, 3], Category::BigStraight), 30);
}

#[test]
#[ignore]
fn big_straight_as_little_straight() {
    assert_eq!(score([6, 5, 4, 3, 2], Category::LittleStraight), 0);
}

#[test]
#[ignore]
fn no_pairs_but_not_a_big_straight() {
    assert_eq!(score([6, 5, 4, 3, 1], Category::BigStraight), 0);
}

#[test]
#[ignore]
fn choice() {
    assert_eq!(score([3, 3, 5, 6, 6], Category::Choice), 23);
}

#[test]
#[ignore]
fn yacht_as_choice() {
    assert_eq!(score([2, 2, 2, 2, 2], Category::Choice), 10);
}
