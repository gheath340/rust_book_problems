pub fn return_first_word() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut word_started = false;
    let mut word_start_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && word_started {
            return &s[word_start_index..i]
        } else if item != b' ' && !word_started {
            word_started = true;
            word_start_index = i;
        }
    }
    &s[word_start_index..]
}