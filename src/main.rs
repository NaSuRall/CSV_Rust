use csv::{Reader, ReaderBuilder};

#[derive(serde::Deserialize, Debug)]
struct Personne {
    nom: String,
    prenom: String,
    email: String,
    tel: i32,
    clases: i32,
}

fn main() {
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

        println!("{:?}", var.nom);
        println!("{:?}", var.prenom);
        println!("{:?}", var.email);
        println!("{:?}", var.tel);
        println!("{:?}", var.clases);
    }
}
