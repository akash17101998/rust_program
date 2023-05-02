pub fn reference_func(){
    // reference is a concept to not change the owner of the variable and fetch the data
    let own = String::from("hi this is owner");
    let refe = length(&own);    // we use & for the reference 
    println!("the length of '{}' is {}",own,refe);
}

fn length(size: &String)->usize{    // &String refer the string type variable 
size.len() 
}