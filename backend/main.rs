fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn findprefix(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return i;
        }
    }

    s.len()
}
fn ir(s: &String) {
    let word = first_word(s);
    let start_word = s[..word];
    let colon = findprefix(s);
    let prefix1 = start_word[..colon]
    let prefix2 = start_word[colon..]
} 