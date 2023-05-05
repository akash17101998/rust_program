
#[derive(Debug)]
pub enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}


pub fn match_func(coin: Coin){
    match coin{
    Coin::Penny =>{
        println!("Lucky");    
        
    }
    Coin::Nickel=> println!("Lucky"),
    Coin::Dime=>  println!("Lucky"),
    Coin::Quarter=>  println!("Lucky"),
    }

    println!("this is match funtion");
}

pub fn new_func(){

// match function with option<t>, it must cover all the possibilities like some and none
pub fn plus(x: Option<i32>)->Option<i32>{
    match x {
        None => None,
        Some(i)=> Some(i+1),
        
    }
}
let one = Some(1);
let six  = plus(one);
let none = plus(None);
println!("{:?}\n{:?}\n{:?}",one,six,none);
}