use std::{collections::{hash_map, HashMap}, path::Ancestors};

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Resposivo".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec!["The musians".to_string(), "The Calling".to_string()],
    );

    show(&table);

    table.insert("teste".to_string(), vec!["a".to_string()]);

    show(&table);

    struct Anime {
        name: &'static str, bechel_pass: bool
    };
    let aria = Anime {name: "Arla: The animation", bechel_pass:true};

    let anime_ref = &aria;

    assert_eq!(anime_ref.name, "Arla: The animation");
    assert_eq!((*anime_ref).name, "Arla: The animation");
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
