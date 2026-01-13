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
    let files = vec!["tableConvert.com_ihxp8w.csv", "data.csv"];
    let json3 = reader_trois(files);
    println!("{:?}", json3);
}

// Fonction qui va pouvoir lire un fichier CSV et en retourner un Vecteur
// avec en paramettre "file_name"
fn reader(file_name: &str) -> Vec<Personne> {
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

    return tab;
}

// Fonction qui va recuperer les fichier du tab Vecteur et les retourner en un seul fichier JSON
fn reader_trois(files: Vec<&str>) -> String {
    let mut resultat: Vec<Personne> = Vec::new();

    for file in files {
        let mut content = reader(file);
        resultat.append(&mut content);
    }

    let pop = serde_json::to_string(&resultat).unwrap();
    return pop;
}

// .append vient prendre tout ce qu'il y a dans content et le met dans resultat et vide content
// appres
