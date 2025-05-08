mod database;

use database::Database;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = &args[1];
    let mut db = Database::open(".rodo.db");
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: rodo add [contents]");
                return;
            }
            println!("Add");
            let contents = &args[2..].join(" ");
            let id = db.read_records().last().map(|r| r.id + 1).unwrap_or(1);
            db.add_todo(&database::Record {
                id,
                contents: contents.to_string(),
            });
        }
        "rm" => {
            if args.len() < 3 {
                println!("Usage: rodo rm [id]");
                return;
            }
            let id = args[2].parse::<i32>().unwrap();
            db.delete_record(id);
            println!("Remove");
        }
        "ls" => {
            println!("List");
            let records = db.read_records();

            if records.is_empty() {
                println!("No records. You can add one with `rodo add [content]`");
                return;
            }

            for record in records {
                println!("{}: {}", record.id, record.contents);
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
