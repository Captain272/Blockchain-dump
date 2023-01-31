fn main() {

    let a:bool=true;
    if a 
    {
        println!("This works");
    }
    else{
        println!("This will not");
    }

    //if statements

   if 32%16==0
   {
        println!("32 is divisible by 16");
   }

   let num=9;


   //find the number is positive or negative
   if num<0
   {
       println!("Number is negative");
   }
   else if num>0
   {
       println!("Number is Positive");
   }

//    String and manipulations.

   let s=String::from("Hello World");
   println!("{}",s);
   //slice from 0 to 2
   println!("slice from 0 to 2: {}",&s[0..2]);
   //slice the String Array
   let Arr=["Mango","Apple","orange"];
   println!("{:?}",Arr);
   //slice the array from 0 to 1
   println!("slice from 0 to 1: {:?}",Arr);
   //Integer Array
   let intArray=[1,2,3,4,5];
   //slice the intArray from 2 to 5
   println!("slice from 2 to 5: {:?}",&intArray[2..5]);
   //two dimenionsonal Array
   let two=[[1,2,3],[2,3,4]];
   println!("slice from 0 to 1: {:?}",&two[0..1]);




}