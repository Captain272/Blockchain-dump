fn main() {
    let str=String::from("Hello world!");
    println!("{}", str);

    // tuple Decleration

    let tuple = ("Hello","World!",1,2,3);

    let (a,b,c,d,e)=tuple;
    
    println!("a={},b={},c={},d={},e={}",a,b,c,d,e);
    //print the tuple for a tuple {:?} is important
    println!("{:?}",tuple);

    let k:bool=true;
    println!("{}",k);

}