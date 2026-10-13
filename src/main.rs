mod file;

fn main() {
    println!("Monkey Lang");
    let file_name = file::read_file_name();
    println!("{}", file_name);

    file::print_file_content(file_name.clone());

    // 1. Read Monkey source [DONE]
    // 2. Lex it
    // 3. Parse it
    // 4. Compile it
    // 5. Output compiled code
}
