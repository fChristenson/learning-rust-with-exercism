pub fn square(s: u32) -> u64 {
    if s <= 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    let mut grain: u64 = 1;

    for _ in 1..s {
        grain = grain * 2;
    }

    return grain;
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;

    for i in 1..65 {
        sum += square(i);
    }

    return sum;
}
