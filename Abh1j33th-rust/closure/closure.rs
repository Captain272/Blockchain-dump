fn main()
{   
    fn function(i:i32) -> i32 {i+1}

    let closure_annot = |i:i32| -> i32 { i+1 };
    let closure_infer = |i    |          i+1 ;
    
    let i=1;

    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annot(i));
    println!("closure_inferred: {}", closure_infer(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());


    let num=3;
    //matching variable num 
    match num{
    //case and statement
    1=>println!("One"),
    2=>println!("Two"),
    3=>println!("Three"),
    _=>println!("Rest of the number"),
    }
    let num=2;
    //match with Several Values
  match num{
  1|3|5|7|9=>println!("Odd"),
  2|4|6|8=>println!("Even"),
  _=>println!("Only one  digit allowed"),
  }
  //boolean match case
    let value = false;
        let coin_side = match value{
            false => "false",
            true => "true",
        };
        println!("Coin Side={}", coin_side);
  //string match case
    let value = "True";
        match value{
            "True" =>println!("True"),
            "False"=> println!("False"),
            _=>println!("anything"),
        }
}