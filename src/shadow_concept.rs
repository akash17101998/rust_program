// in this we are doing the shadowing of x variable means declare the same variable name as the previous one 
pub fn shadow_func(){
    let x = 5;
    let x = x+1;
    println!("{}",x);

// in this we make a mutable variable so that we can change the value of var., without mut it will give error 
    let mut a = 10;
    a = 15;
    println!("{}",a);

}