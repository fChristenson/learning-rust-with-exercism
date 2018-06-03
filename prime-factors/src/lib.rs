pub fn factors(n: u64) -> Vec<u64> {
    let mut results = vec![];

    if n <= 1 {
        return results;
    }

    for number in 2..n + 1 {
        if is_prime(number) && n % number == 0 {
            results.push(number);
            let remainder = n / number;
            let mut factors: Vec<u64> = factors(remainder);
            results.append(&mut factors);
            break;
        }
    }

    return results;
}

fn is_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    return n != 1;
}
