pub fn pri(c: &char) -> u32 {
    let ascii = u32::from(c.clone());
    match c {
        'a'..='z' => ascii - u32::from('a') + 1,
        'A'..='Z' => ascii - u32::from('A') + 1 + 26,
        _ => panic!("invalid character"),
    }
}
