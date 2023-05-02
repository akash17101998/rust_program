// let take a very simple example
pub fn owner_func(){
    // these are the literals and value is hardcoded with the variable that are store on stack
    let x = "hello";
    let y = x;
    println!("x: {}",x);
    println!("y: {}",y);
    
    // we can use make a mutable String 
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    println!("{}",s1);
    // let assign the value of s1 to s2 and if we print the value of s2 it execute without any error 
    let s2 = s1.clone();    // so basically we use .clone() method to deeply copy the data
    println!("{}",s2);
    // but if we execute the value of s1 it will show error, that's the ownership error because the ownership transfer to s2
    println!("{}",s1);    

    // so basically what happen over here, s1 contain three field ie. ptr,length and capcity .The ptr point to the index of string and index
    // contain the value. s1 is stored on the stack and the string is stored on the heap memory.
    // When we try assign the s1 to s2 we actually copy the ptr,length and capacity of the stack memory not the heap memory
}