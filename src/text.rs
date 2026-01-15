use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn main() {
    write_text();
    reader_text("test.txt");
}

pub fn reader_text(file_name: &str) -> String {
    let texte = fs::read_to_string(file_name).expect("File not found");
    println!("{}", texte);
    return texte;
}

pub fn write_text() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("test.txt")?;
    file.write_all(b"Voici le texte ajouter");
    Ok(())
}
