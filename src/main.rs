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
    // Fonction READER va lire le fichier et le retourner
    // let json1 = reader("tableConvert.com_ihxp8w.csv");
    // println!("json 1 = {}", json1);
    // let json2 = reader("data.csv");
    // println!("json 2 = {}", json2);

    // Fonction READER_DEUX va lire les fichier et retourner un grand JSON
    // let json = reader_deux("tableConvert.com_ihxp8w.csv", "data.csv");
    // println!("{:#?}", json);

    let files = vec!["tableConvert.com_ihxp8w.csv", "data.csv"];
    reader_trois(&files);
}

// Fonction reader qui va avoir en paramettre "file_name" et retourner un json
// cette fonction va annalyser le fichier CSV et le retoutner en format JSON

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
}

// Fonction reader_deux va avoir 2 paramettre : file_1 et file_2 : qui vont etre les fichier CSV
// cette fonction va reunir les deux fichier CSV et en faire UN seul grand JSON

fn reader_deux(file_1: &str, file_2: &str) -> String {
    let reader1 = reader(file_1);
    let reader2 = reader(file_2);

    let mut next_vec: Vec<String> = Vec::new();

    next_vec.push(reader1);
    next_vec.push(reader2);

    serde_json::to_string(&next_vec).unwrap()
}

// Fonction reader_trois va avoir en paramettre un Tableau pour y mettre autant de fichier
// il va iterer sur le tableau recuperer et annalyser tout les fichier
// et en faire un gros JSON
//
fn reader_trois(files: &Vec<&str>) {
    for valeur in files.iter() {
        let value = reader(&valeur);
        println!("{}", value);
    }
}
