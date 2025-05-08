use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::io::{Seek, Write};

pub struct Database {
    pub file: File,
}

pub struct Record {
    pub id: i32,
    pub contents: String,
}

impl Database {
    pub fn open(filename: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        Self { file }
    }
    pub fn add_todo(&mut self, content: &Record) -> i32 {
        let line = format!("{}|{}\n", content.id, content.contents);
        writeln!(self.file, "{}", line).unwrap();
        println!("item added: {}", content.contents);
        0
    }

    pub fn read_records(&mut self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);

        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| parse_record_line(&line))
            .collect()
    }

    pub fn delete_record(&mut self, id: i32) {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();

        let line = lines.find(|(_, line)| {
            let record = parse_record_line(line.as_ref().unwrap());
            record.id == id
        });

        match line {
            Some((index, _)) => {
                let contents = fs::read_to_string(".rodo.db").unwrap();

                let new_contents = contents
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != index)
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("\n");

                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();
            }
            None => {
                println!("no record with id {} found", id);
                return;
            }
        }
    }
}

pub fn parse_record_line(line: &str) -> Record {
    let fields: Vec<&str> = line.split("|").collect();
    if fields.len() == 1 {
        return Record {
            id: 0,
            contents: fields[0].to_string(),
        };
    }
    let content = fields[1].to_string();
    Record {
        id: fields[0].parse().unwrap(),
        contents: content,
    }
}
