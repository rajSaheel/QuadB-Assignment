pub fn check_palindrome(str: &str) -> bool {
    let mut i = 0;
    let mut j = str.len() - 1;

    while i <= j {
        if str.chars().nth(i) != str.chars().nth(j) {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
