use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: cr -- <search_term> <filename>");
    }

    let query = &args[1];
    let file_path = &args[2];

    dbg!(args);
}
