// read_char()
// next_token()
// skip_whitespace()

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: (),
        }
    }
}
pub fn read_char() {
    println!("Reading File")
}
