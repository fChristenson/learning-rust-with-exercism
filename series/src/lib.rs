pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); 6];
    }

    if len == digits.len() {
        return vec![digits.to_string()];
    }

    if len > digits.len() {
        return vec![];
    }

    let strings: Vec<String> = digits
        .split("")
        .filter(|c| c != &"")
        .map(|c| c.to_string())
        .collect();

    let mut results: Vec<String> = vec![];

    for i in 0..strings.len() {
        if i + len > strings.len() {
            break;
        }
        let chars: String = strings[i..i + len].join("");
        results.push(chars);
    }

    return results;
}
