use csv::ReaderBuilder;
use serde::Serialize;

#[derive(serde::Deserialize, Debug, Serialize, Clone)]
struct Personne {
    nom: String,
    prenom: String,
    email: String,
    tel: String,
    clases: String,
}

fn main() {
    // let mut tab: Vec<Personne> = Vec::new();
    let json1 = reader("tableConvert.com_ihxp8w.csv");
    println!("json 1 = {}", json1);

    let json2 = reader("data.csv");
    println!("json 2 = {}", json2);
}

fn reader(file_name: &str) -> String {
    let mut builder = ReaderBuilder::new();
    builder
        .double_quote(false)
        .comment(Some(b'-'))
        .delimiter(b',');

    let mut my_reader = builder.from_path(file_name).expect("File not found");

    let mut tab: Vec<Personne> = Vec::new();

    for record in my_reader.deserialize() {
        let var: Personne = record.unwrap();
        tab.push(var);
    }

    serde_json::to_string(&tab).unwrap()
    //println!("{}", j);
}

// faire un fonction qui va pouvoir lire deux fichier et les annalyser pour resortir un seul JSON
#[warn(dead_code)]
fn reader_deux() {}
