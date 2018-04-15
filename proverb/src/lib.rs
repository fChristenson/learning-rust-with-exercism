pub fn build_proverb(list: Vec<&str>) -> String {
    let mut result = String::new();

    for i in 0..list.len() {
        let first = list.get(i).unwrap();
        let maybe_last = list.get(i + 1);
        match maybe_last {
            Some(last) => {
                let string = format!("{}\n", get_string(first, last));
                result.push_str(&string)
            }
            _ => result.push_str(&get_final_string(list.get(0).unwrap())),
        }
    }

    return result;
}

fn get_string(a: &str, b: &str) -> String {
    return format!("For want of a {} the {} was lost.", a, b);
}

fn get_final_string(a: &str) -> String {
    return format!("And all for the want of a {}.", a);
}
