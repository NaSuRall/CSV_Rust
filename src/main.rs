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
    // let json1 = reader("tableConvert.com_ihxp8w.csv");
    //println!("json 1 = {}", json1);

    // let json2 = reader("data.csv");
    // println!("json 2 = {}", json2);

    let json = reader_deux("tableConvert.com_ihxp8w.csv", "data.csv");
    println!("{}", json);
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
fn reader_deux(file_1: &str, file_2: &str) -> String {
    let reader1 = reader(file_1);
    let reader2 = reader(file_2);

    let mut next_vec: Vec<String> = Vec::new();

    next_vec.push(reader1);
    next_vec.push(reader2);

    serde_json::to_string(&next_vec).unwrap()
}

// j'ai pas fait mon commit aled il est trop tard je continue mais apres 00h lol
