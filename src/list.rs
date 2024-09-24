use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Hash, Deserialize, Serialize, Debug)]
struct Todo {
    id: String,
    description: String,
    date: String,
    category: String,
    amount: String,
}

pub fn listall() -> Result<(), Box<dyn Error>> {
    let todos = read_to_string("/home/charan/Downloads/todo.json")?;

    println!(
        "{}, {}, {}, {}, {}",
        "Id".bold().red(),
        "Description".bold().red(),
        "Amount".bold().red(),
        "Category".bold().red(),
        "Date".bold().red()
    );

    for (_index, (_key, transaction)) in todos.iter().enumerate() {
        println!(
            "{}, {}, {}, {}, {}",
            transaction.id.green(),
            transaction.description.green(),
            transaction.amount.green(),
            transaction.category.green(),
            transaction.date.green()
        );
    }

    Ok(())
}

fn read_to_string(path: &str) -> Result<HashMap<String, Todo>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let u: HashMap<String, Todo> = serde_json::from_reader(reader)?;

    Ok(u)
}
