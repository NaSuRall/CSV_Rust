use csv::Reader;

fn main() {
    // Nom du fichier a lite le fichier CSV
    let file_name = "tableConvert.com_ihxp8w.csv";
    // Lire le Fichier avec la librarie CSV de RUST en parametre le nom du fichier
    let result = Reader::from_path(file_name);

    // Mise en place d'une erreur au cas ou ca ne marche pas
    if result.is_err() {
        println!("Impossible de lire le Fichier CSV, il n'existe pas ");
        std::process::exit(9);
    }
    // unwrap les données de result stocker dans my_reader
    let mut my_reader = result.unwrap();
    // Boucler les données de my_reader
    for record in my_reader.records() {
        // déclarer en variable record.unwrap car multiple utilisation
        let record = record.unwrap();

        // afficher dans le terminal les result des columns
        println!("Les Noms : {}", record.get(0).unwrap());
        println!("Les Prénom : {}", record.get(1).unwrap());
    }
}
