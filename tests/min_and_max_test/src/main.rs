use min_and_max::min_and_max;

fn main() {
    println!(
        "Minimum and maximum are: {:?}",
        min_and_max(9, 2, 4)
    );
}

#[test]
fn test_min_and_max() {
    let (min, max) = min_and_max(0, 0, 0);

    assert_eq!(min, 0);
    assert_eq!(max, 0);

    let (min, max) = min_and_max(1, 2, 3);

    assert_eq!(min, 1);
    assert_eq!(max, 3);

    let (min, max) = min_and_max(-1, -2, -3);

    assert_eq!(min, -3);
    assert_eq!(max, -1);

    let (min, max) = min_and_max(14, -12, 3);

    assert_eq!(min, -12);
    assert_eq!(max, 14);
}
