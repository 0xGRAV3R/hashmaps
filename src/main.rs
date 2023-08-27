use std::collections::HashMap;

fn main() {
    let mut my_hash_map: HashMap<String, i32> = HashMap::new();
    
    my_hash_map.insert(String::from("key1"), 42);
    my_hash_map.insert(String::from("key2"), 87);

    println!("Hello, hashmaps!");
}
