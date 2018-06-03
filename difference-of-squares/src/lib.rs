pub fn square_of_sum(n: usize) -> usize {
    let mut numbers = 0;

    for i in 0..n + 1 {
        numbers += i;
    }

    return numbers.pow(2);
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut numbers = vec![];

    for i in 0..n + 1 {
        numbers.push(i);
    }

    let sum: usize = numbers.iter().fold(0, |acc, number| acc + number.pow(2));
    return sum;
}

pub fn difference(n: usize) -> usize {
    return square_of_sum(n) - sum_of_squares(n);
}
