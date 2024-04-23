fn longest_common_prefix(strings: Vec<String>) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_string = &strings[0];

    for (i, char) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(c) = string.chars().nth(i) {
                if c != char {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(char);
    }

    prefix
}
