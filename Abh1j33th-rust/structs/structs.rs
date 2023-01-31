// The basic flow of stucts in rust

// let instance_name = Name_of_structure {
//     field1:value1,
//     field2:value2,
//     field3:value3
//  }; 
//  //NOTE the semicolon
//  Syntax: Accessing values in a structure
//  Use the dot notation to access value of a specific field.
//  instance_name.field1

struct Marvel {
    name:String,
    power:String,
    age:u32
}

fn main(){
    let mrvl_1=Marvel{
        name:String::from("Captain_America"),
        power:String::from("SuperHuman & shield"),
        age:150
    };
    println!("Name is :{} power is {} age is {}",mrvl_1.name,mrvl_1.power,mrvl_1.age);
}