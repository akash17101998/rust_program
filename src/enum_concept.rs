pub fn enum_func(){
    #[derive(Debug)]
    enum IpAddrKind{
        V4,
        V6,
    }
#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr{
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let office = IpAddr{
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

println!("{:?}",home);
println!("{:?}",office);


// we can also define enum 

#[derive(Debug)]
enum Message{
    Move(String),
    value(u32,u32),
    Quit,
}

let user1 = Message::Move(String::from("yes"));
println!("{:?}",user1);
let user2 = Message::value((12), (10));
println!("{:?}",user2);
let user3 = Message::Quit;
println!("{:?}",user3);


}