// importing library

use std::collections::HashMap;


fn main(){
    let mut map = HashMap::new();

    // Inserting elements into map
    map.insert(1,"one");
    map.insert(2,"two");
    map.insert(3,"Three");
    map.insert(4,"Four");
    map.insert(5,"Five");

    println!("{:?}",map);

    for (key, value) in map.iter(){
        println!("{} {}",key,value);
    }

    // let value= map.get(&5); This can also be done.
    println!("value of 5 is ={:?}",map.get(&5));
    //remove from map
    map.remove(&5);
    //after removing print the hashmap
    println!("{:?}",map);





}