

pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
   
    let min = (nb_2). min (nb_3). min (nb_1);
    let max = (nb_1). max (nb_3). max (nb_2);
    return (min, max)
}

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