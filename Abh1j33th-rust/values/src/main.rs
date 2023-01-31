fn main() {
    println!("{} {}","Basic","Concept of rust");

    // Add two ints

    let mut x=2;
    // If we didn't specify mut here this may lead to error. Coz
    // we are overwriting the value of x.
    let mut y=3;

    x=x+y;
    println!("{}",x);


    x=2;
    y=3;
    //Addition Operation
    println!("x+y={}",x+y);
    //subtration Operation
    println!("y-x={}",y-x);
    //Multiplying Operation
    println!("x*y={}",x*y);
    //Division Operation
    println!("y/x={}",y/x);

    let a=true;
    let b=false;
    //boolean Operations
    println!("a={}\nb={}",a,b);
    println!("a & b = {}",a&b);
    println!("a || b = {}",a || b);
   //float values
   println!("7.0+2.1={}",7.0+2.1);

//    String Operations
    let mut word="Kingu_world";

    println!("{}",word);

    let mut foo=String::new();

    foo.push_str("Kingu_world");
    foo.push_str("Hello");

    println!("{}",foo);
    println!("{}",&foo[0..9]);
    // foo.pop_str();
    // println!("{}",foo);





}
