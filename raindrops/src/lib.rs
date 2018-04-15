pub fn raindrops(n: usize) -> String {
    let mut str_vec = vec![];

    if n % 3 == 0 {
        str_vec.push("Pling");
    }

    if n % 5 == 0 {
        str_vec.push("Plang");
    }

    if n % 7 == 0 {
        str_vec.push("Plong");
    }

    if str_vec.len() <= 0 {
        return n.to_string();
    } else {
        return str_vec.join("");
    }
}
