use std::{collections::HashMap, io::Read, str::FromStr};
fn main() {
    let action: String = std::env::args().nth(1).expect("Specify an action.");
    let item: String = std::env::args().nth(2).expect("Specify an item.");

    let mut todo = Todo::new().expect("Initialization of db failed");
    
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved."),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}

struct Todo {
    //store key - val pairs using HashMap
    map: HashMap<String, bool>,
}

impl Todo {
    /* 
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map:HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }
    */
    fn new() -> Result<Todo, std::io::Error> {
        //open file
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("db.txt")?;
        //read content into a string
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        //allocate empty HashMap
        let mut map: HashMap<String, bool> = HashMap::new();

        //loop over each line in file
        for entries in content.lines() {
            //split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No key");
            let val = values.next().expect("No value");
            //insert them into HashMap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        //return Ok
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        //insert new item into our map
        //passing true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}