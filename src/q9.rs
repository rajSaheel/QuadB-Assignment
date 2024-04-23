// using inbuilt function
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

//using loop
fn reverse_string_(input: &str) -> String {
    let mut reversed = String::new();
    let mut i = input.len();
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        reversed.push(ch);
        i -= 1;
        while i > 0 {
            reversed.push(chars.clone().nth(i - 1).unwrap());
            i -= 1;
        }
    }

    reversed
}
