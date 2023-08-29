# hashmaps
Hashmaps allow you to store key-value pairs. Those keys and value can be of any type. Also it uses a hashing function to determine how to place those keys and values in memory.
Hashmaps allow you to store key-value pairs. Those keys and value can be of any type. Also it uses a hashing function to determine how to place those keys and values in memory. In order to create a new hashmaps, first we need to bring the hashmap type into scope from the standard library.

Examples:
1) Bring hashmap type into scope from the std lib:
use std::collections::HashMap;

2) Creating a new HashMap:
let mut my_hash_map: HashMap<String, i32> = HashMap::new();



You can reasd the tutorial here:
https://oceansthreads.substack.com/p/hashmaps-in-rust
