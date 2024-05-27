mod db;
use std::io::Write;

use db::DB;

fn main() {
    println!("[nodenote]");
    let mut db = DB::new();
    let mut input_buf = String::new();

    loop {
        input_buf.clear();
        if let Some(ctx) = db.get_context() {
            print!("#[{}] ", ctx.as_str());
        }else{
            print!("#> ", );
        }

        std::io::stdout().flush().expect("Error occured at stdout flush.");
        std::io::stdin().read_line(&mut input_buf).expect("Error occured at readline.");

        let input = if let Some (r) = input_buf.split_once(' '){
            r
        } else {
            match input_buf.trim() {
                "help" | "?" => {
                    println!(include_str!("help.txt"));
                },

                "back" | "b" => db.back(),

                "root" | "r" => db.back_to_root(),

                "list" | "ls" | "l" => db.print_links(),

                _ => ()
            };
            continue;
        };

        let term = input.1.trim();
        match input.0 {
            "addenter" | "ae" => {
                let r = db.add_term(term);
                db.set_context(r);
            },

            "add" | "a" => {
                db.add_term(term);
            },

            "push" | "p" => {
                db.append(term);
            },

            "pushenter" | "pe" => {
                if let Some(r) = db.append(term) {
                    db.set_context(r);
                }
            },

            "enter" | "cd" | "e" => {
                if term == ".." {
                    db.back();
                }else{
                    db.search(term.to_string());
                }
            },

            "subsearch" | "ss" => {
                db.subsearch(term);
            },

            "save" => if let Err(e) = db.save(term) {
                println!("Failed to save: {}", e.to_string());
            },

            "load" => if let Err(e) = db.load(term) {
                println!("Failed to save: {}", e.to_string());
            },

            _ => {
                println!("Invalid command.");
            }

        }
    }

    
}