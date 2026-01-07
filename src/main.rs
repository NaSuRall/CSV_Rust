// use csv::Reader;
//
// fn main() {
//     // Nom du fichier a lite le fichier CSV
//     let file_name = "tableConvert.com_ihxp8w.csv";
//     // Lire le Fichier avec la librarie CSV de RUST en parametre le nom du fichier
//     let result = Reader::from_path(file_name);
//
//     // Mise en place d'une erreur au cas ou ca ne marche pas
//     if result.is_err() {
//         println!("Impossible de lire le Fichier CSV, il n'existe pas ");
//         std::process::exit(9);
//     }
//     // unwrap les données de result stocker dans my_reader
//     let mut my_reader = result.unwrap();
//     // Boucler les données de my_reader
//     for record in my_reader.records() {
//         // déclarer en variable record.unwrap car multiple utilisation
//         let record = record.unwrap();
//
//         // afficher dans le terminal les result des columns
//         println!("Les Noms : {}", record.get(0).unwrap());
//         println!("Les Prénom : {}", record.get(1).unwrap());
//         println!("la classe : {}", record.get(4).unwrap());
//     }
//
//     // lire l'entrer Utilsateur et faire un choix d'export ou non
//     println!("Voulez vous exporter en Format Json ? ");
//     println!("OUI ou NON");
//     let mut input = String::new();
//     let choix = std::io::stdin().read_line(&mut input).unwrap();
//
//     // Regarder les vidéos pour JSON serializer et inversement
//     // pas beaucoup mais hasoul on est dimanche ok
//
// }
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "id": 1,
            "title": "Titre 1".to_owned(),
            "completed": false,
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    Ok(())
}

// regarder et configurer Dioxus pas grand chose de plus demain mise a 0 commencement du projet
// Dev le lecteur de CSV et exportrer transformer le dossier en json
// objectif

// 07/01/2026 j'ai fait la configuration de mon serveur de dev Ubuntu serveur pas eu trop le temps
// de faire du rest c'est mb demain je me donne a fond ok
//
// objectif de demain faire le CSV convertisseur en JSON
