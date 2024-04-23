fn find_shortest_word(sentence: &str) -> Option<&str> {
    let mut shortest_word: Option<&str> = None;
    for word in sentence.split_whitespace() {
        shortest_word = match shortest_word {
            Some(shortest) => {
                if word.len() < shortest.len() {
                    Some(word)
                } else {
                    Some(shortest)
                }
            }
            None => Some(word),
        };
    }
    shortest_word
}
