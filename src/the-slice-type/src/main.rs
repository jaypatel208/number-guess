fn main() {
    let mut s = String::from("hello world");
    let _word = first_word(&s);
    println!("First word is {}", _word);
    s.clear();

    let song_name = String::from("Rait zara si");
    let rait = &song_name[..3]; // ..3 & 0..3 both are same
    let zara = &song_name[5..8];
    let si = &song_name[10..]; // 10.. is same as 10..len // .. equals to 0..len

    print!("Song name {} {} {}", rait, zara, si);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
