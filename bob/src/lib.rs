pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if is_shouting(trimmed) && is_question(trimmed) {
        return "Calm down, I know what I'm doing!";
    }

    if trimmed == "" {
        return "Fine. Be that way!";
    }

    if is_shouting(trimmed) {
        return "Whoa, chill out!";
    }

    if is_question(trimmed) {
        return "Sure.";
    }

    return "Whatever.";
}

fn is_question(message: &str) -> bool {
    let letter = message.chars().last();

    match letter {
        Some(letter) => letter.to_string() == "?",
        _ => false,
    }
}

fn is_shouting(message: &str) -> bool {
    let letters: String = message
        .chars()
        .filter(|c| !is_special_character(*c))
        .collect();

    if letters.len() <= 0 {
        return false;
    }

    return letters.chars().all(is_uppercase);
}

fn is_special_character(letter: char) -> bool {
    return is_lowercase(letter) && is_uppercase(letter);
}

fn is_uppercase(letter: char) -> bool {
    return letter.to_string() == letter.to_uppercase().nth(0).unwrap().to_string();
}

fn is_lowercase(letter: char) -> bool {
    return letter.to_string() == letter.to_lowercase().nth(0).unwrap().to_string();
}
