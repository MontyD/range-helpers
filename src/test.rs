use super::*;

#[test]
fn it_will_add_single_range() {
    let mut ranges = SortedRanges::new();
    ranges.insert(&(12, 15));

    assert_eq!(ranges.drain(..).collect::<Vec<(i32, i32)>>(), vec![(12, 15)]);
}

#[test]
fn it_will_join_ranges() {
    let mut ranges = SortedRanges::new();
    ranges.insert(&(15, 25));

    ranges.insert(&(12, 20));

    assert_eq!(ranges.drain(..).collect::<Vec<(i32, i32)>>(), vec![(12, 25)]);
}

#[test]
fn it_will_join_many_ranges() {
    let mut ranges = SortedRanges::new();
    ranges.insert(&(1, 10));
    ranges.insert(&(10, 25));
    ranges.insert(&(35, 40));
    ranges.insert(&(41, 51));
    ranges.insert(&(27, 27));

    assert_eq!(ranges.drain(..).collect::<Vec<(i32, i32)>>(), vec![
        (1, 25),
        (27, 27),
        (35, 51),
    ]);
}