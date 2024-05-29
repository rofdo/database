use std::{collections::HashMap, vec};

struct Database {
    name: String,
    data: HashMap<String, String>,
}

impl Database {
    fn new(name: String) -> Database {
        Database {
            name,
            data: HashMap::new(),
        }
    }
}

fn main() {
    let mut databases: HashMap<String, Database> = HashMap::new();
    loop {
        let input = read_input();
        // ensure ends with ;
        if input.trim().ends_with(";") == false {
            println!("Error: Command must end with ;");
            continue;
        }
        // seperate by ;
        let mut commands: Vec<&str> = input.split(";").collect();
        // remove last empty string
        commands.pop();
        commands.iter().for_each(|command| {
            let command = command.trim();
            let command_parts: Vec<&str> = command.split(" ").collect();
            match command_parts[0] {
                "CREATE" => {
                    let db_name = match command_parts.get(1) {
                        Some(name) => name,
                        None => {
                            println!("Error: Database name is required");
                            return;
                        }
                    };
                    // check if already exists
                    match databases.contains_key(db_name.clone()) {
                        true => {
                            println!("Error: Database already exists");
                            return;
                        }
                        false => {
                            let db = Database::new(db_name.to_string());
                            databases.insert(db_name.to_string(), db);
                            println!("Database \"{}\" created successfully", db_name);
                        }
                    }
                }
                "DROP" => {
                    let db_name = match command_parts.get(1) {
                        Some(name) => name,
                        None => {
                            println!("Error: Database name is required");
                            return;
                        }
                    };
                    match databases.remove(*db_name) {
                        Some(_) => {
                            println!("Database \"{}\" dropped successfully", db_name);
                        }
                        None => {
                            println!("Error: Database does not exist");
                        }
                    }
                }
                bad_command => {
                    println!("Error: Invalid command \"{}\"", bad_command);
                }
            }
        });
        

    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

