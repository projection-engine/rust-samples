// Nested / Add value / Print / add key
use std::collections::HashMap;

#[derive(Debug)]
struct Test {
  nested_hashmap: HashMap<String, HashMap<i32, Vec<(i32)>>>,
}

fn main() {
  let mut test = Test { nested_hashmap: HashMap::new() };

  test.nested_hashmap.entry("Foo".into()).or_insert(HashMap::new());
  
  for (_, mut thing) in test.nested_hashmap.iter_mut() {
    thing.entry(1).or_insert(vec![666]);
  }
  
  println!("{:#?}", test);
}
// Nested / Add value / Print / add key
// Thanks to https://users.rust-lang.org/t/nested-hashmaps/3284    
    
    
    
