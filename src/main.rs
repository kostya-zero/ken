use std::{env::args, io::Read, process::exit};

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() == 1 {
        println!("Nothing to search.");
        exit(1);
    }

    if args.len() > 2 {
        println!("Too many arguments.");
        exit(1);
    }

    let text = args[1].clone();
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).unwrap();

    if content.is_empty() {
        exit(0);
    }

    let lines = content.lines();
    for (id, line) in lines.enumerate() {
        if line.contains(&text) {
            let modified = line.replace(&text, &format!("\x1b[91m\x1b[1m{text}\x1b[0m"));
            println!(" {}:  {modified}", id + 1);
        }
    }
}
