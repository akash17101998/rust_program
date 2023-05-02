pub fn struct_func(){
    // simple struct program 

    #[derive(Debug)]
    struct User{
        fname : String,
        lname : String,
        email_id: String,
    }

    let user = User{
        fname: String::from("akash"),
        lname: "singh".to_string(),
        email_id: "abc@gmail.com".to_string(),
    };
    println!("{} \n{}\n{}",user.fname,user.email_id,user.lname);
    println!("{:?}",user);

}


// in this program we use reference to point the variable because without reference the owner will tranfer which causes error
struct Rectangle{
    width: u32,
    height: u32,
}
pub fn area_of_rectangle(){
    
let rectangle = Rectangle{
    width:20,
    height:30,
};
println!("the area of rectangle is {}",area(&Rectangle{width:10,height:20}));
println!("the area of rectangle is {}",area(&rectangle));

// println!("the area of rectangle is {}",area(rectangle));   
// println!("the area of rectangle is {}",rectangle.width);  // this will give error if we remove & from area function because of ownship 

}

fn area(rect1: &Rectangle)->u32{
    rect1.width*rect1.height
    }