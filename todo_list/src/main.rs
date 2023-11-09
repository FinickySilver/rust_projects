use std::collections::HashMap;
fn main() {
    let action: String = std::env::args().nth(1).expect("Specify an action.");
    let item: String = std::env::args().nth(2).expect("Specify an item.");

    let mut todo = Todo {
        map: HashMap::new(),
    };
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