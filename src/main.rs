use csv::{Reader, ReaderBuilder};
use serde::Serialize;

#[derive(serde::Deserialize, Debug, Serialize, Clone)]
struct Personne {
    nom: String,
    prenom: String,
    email: String,
    tel: i32,
    clases: i32,
}

fn main() {
    let mut tab: Vec<Personne> = Vec::new();

    let file_name = "tableConvert.com_ihxp8w.csv";
    let mut builder = ReaderBuilder::new();
    builder
        .double_quote(false)
        .comment(Some(b'-'))
        .delimiter(b',');

    let result = builder.from_path(file_name);

    if result.is_err() {
        println!("File not Find");
        std::process::exit(9);
    }

    let mut my_reader: Reader<std::fs::File> = result.unwrap();

    for record in my_reader.deserialize() {
        let var: Personne = record.unwrap();
        tab.push(var);
    }

    // Faire en sorte D'analyser 2 CSV et les faire sortir en un seul JSON

    let j = serde_json::to_string(&tab).unwrap();
    println!("{}", j);
}
