use pythagorean_triplet::*;
use std::collections::HashSet;

#[test]
fn triplets_whose_sum_is_12() {
    let output = find(12);
    let expected: HashSet<_> = [[3, 4, 5]].iter().cloned().collect();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn triplets_whose_sum_is_108() {
    let output = find(108);
    let expected: HashSet<_> = [[27, 36, 45]].iter().cloned().collect();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn triplets_whose_sum_is_1000() {
    let output = find(1_000);
    let expected: HashSet<_> = [[200, 375, 425]].iter().cloned().collect();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn no_matching_triplets_for_1001() {
    let output = find(1_001);
    assert!(output.is_empty());
}

#[test]
#[ignore]
fn returns_all_matching_triplets() {
    let output = find(90);
    let expected: HashSet<_> = [[9, 40, 41], [15, 36, 39]].iter().cloned().collect();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn several_matching_triplets() {
    let output = find(840);
    let expected: HashSet<_> = [
        [40, 399, 401],
        [56, 390, 394],
        [105, 360, 375],
        [120, 350, 370],
        [140, 336, 364],
        [168, 315, 357],
        [210, 280, 350],
        [240, 252, 348],
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn triplets_for_large_number() {
    let output = find(30_000);
    let expected: HashSet<_> = [
        [1_200, 14_375, 14_425],
        [1_875, 14_000, 14_125],
        [5_000, 12_000, 13_000],
        [6_000, 11_250, 12_750],
        [7_500, 10_000, 12_500],
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(output, expected);
}
