fn main() {
    let mut elements=[1,2,3,4,5,6];

    for a in elements
    {
        println!("{}", a);
    }
    // print a array in single line
    println!("{:?}", elements);

    // Reverse array in
    elements.reverse();

    // Array slice
    let slice = &elements[0..4];

    println!("{:?}",slice);

    //update array value
   elements[0]=22;

   println!("updated array: {:?}",elements);
   //two dimensional Array in rust
   let two=[[1,2,3],[4,5,6]];
   //print two dimensional array;
   println!("{:?}",two);

   let mut a:[i32;5];
   a=[1,2,3,4,5];
   println!("{:?}",a);
   //two dimensional Array
   let mut two:[[i32;3];3];
   two=[[1,2,3],[4,5,6],[7,8,9]];
   //print two dimensional Array
   println!("{:?}",two);



}