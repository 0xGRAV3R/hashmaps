# hashmaps
Hashmaps allow you to store key-value pairs. Those keys and value can be of any type. Also it uses a hashing function to determine how to place those keys and values in memory.
Hashmaps allow you to store key-value pairs. Those keys and value can be of any type. Also it uses a hashing function to determine how to place those keys and values in memory. In order to create a new hashmaps, first we need to bring the hashmap type into scope from the standard library.

use std::collections::HashMap;
Next is create a new HashMap: You can create a new hash map using the HashMap::new() constructor:

let mut my_hash_map: HashMap<String, i32> = HashMap::new();
In this example, the hash map will store keys of type String and values of type i32.

Next is Inserting and Updating Values: You can insert or update values in the hash map using the insert method:

my_hash_map.insert(String::from("key1"), 42); my_hash_map.insert(String::from("key2"), 87);
Next is accessing values: You can access values in the hash map using the get method:

if let Some(value) = my_hash_map.get("key1") {
    println!("Value: {}", value);
} else {
    println!("Key not found");
}
You can also do iterating over pairs: You can iterate over the key-value pairs in the hash map using a for loop:

for (key, value) in &my_hash_map {
    println!("Key: {}, Value: {}", key, value);
}
Removing Values: You can remove values from the hash map using the remove method:

my_hash_map.remove("key1");
Checking for Key Existence: You can check if a key exists in the hash map using the contains_key method:

if my_hash_map.contains_key("key1") {
    println!("Key exists");
} else {
    println!("Key does not exist");
}
Remember that hash maps in Rust follow ownership rules. If you want to store values that implement the Copy trait (like integers), you can do so directly. However, if you want to store values that don't implement Copy, you might need to use owned types (like String) or references.

Here's a full example that puts it all together:

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
Remember to add this code to a Rust source file and use the cargo run command to execute it.
