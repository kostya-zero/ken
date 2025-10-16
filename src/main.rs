use std::{io::Read, process::exit};

use clap::Parser;

use crate::{cli::Cli, terminal::highlight};

mod cli;
mod terminal;

fn main() {
    let cli = Cli::parse();

    if cli.pattern.is_none() {
        println!("Nothing to search.");
        exit(1);
    }

    let text = cli.pattern.unwrap();
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).unwrap();

    if content.is_empty() {
        exit(0);
    }

    let lines = content.lines();
    let hightlight_color = cli.hightlight_color.as_str();
    for (id, line) in lines.enumerate() {
        if line.contains(&text) {
            let modified = highlight(&text, hightlight_color.into());
            let modified_display = line.replace(&text, modified.as_str());
            println!(" {}:  {modified_display}", id + 1);
        }
    }
}
