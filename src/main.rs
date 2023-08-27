use std::collections::HashMap;

fn main() {
    let mut my_hash_map: HashMap<String, i32> = HashMap::new();

    my_hash_map.insert(String::from("key1"), 42);
    my_hash_map.insert(String::from("key2"), 87);

    if let Some(value) = my_hash_map.get("key1") {
        println!("Value: {}", value);
    } else {
        println!("Key not found");
    }

    for (key, value) in &my_hash_map {
        println!("Key: {}, Value: {}", key, value);
    }

    my_hash_map.remove("key1");

    if my_hash_map.contains_key("key1") {
        println!("Key exists");
    } else {
        println!("Key does not exist");
    }
}





