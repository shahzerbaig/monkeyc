use std::error::Error;

mod file;
mod lexer;
mod token;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Monkey Lang");
    let filename = file::read_file_name()?;
    lexer::read_char();

    println!("{}", filename);
    let input = file::read_file_content(filename)?;

    let mut lexer = lexer::Lexer::new(&input);

    loop {
        let token = lexer.next_token();
        println!("{}: {}", token.kind, token.literal);

        if token.kind == token::TokenKind::EOF {
            break;
        }
    }
    //file::print_file_content(file_name.clone());

    // 1. Read Monkey source [DONE]
    // 2. Lex it
    // 3. Parse it
    // 4. Compile it
    // 5. Output compiled code

    Ok(())
}
