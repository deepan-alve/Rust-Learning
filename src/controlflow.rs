pub fn run () {
    // loop
    let mut str = String::from("hello world");
    for letter in str.split_whitespace() {
        println!("{}", letter);
    }
    for letter in str.chars()
    {
        println!("{}", letter);
    }
}