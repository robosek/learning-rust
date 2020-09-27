use std::process::exit;

enum IpAddrKind{
    V4,
    V6
}

struct IpAddress{
    Type: IpAddrKind,
    Address: String
}

enum IpAddress2{
    V4(i32,i32,i32,i32),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Message{
    fn call(&self){

    }
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    let home = IpAddress{
        Type: v4,
        Address: String::from("192.168.1.1")
    };

    let address = IpAddress2::V4(192,168,1,1);

    let message = Message::Write(String::from("Hello world!"));

    match &message {
        Message::Quit => exit(1),
        Message::Move {x,y} => println!("This is a point x: {}, y:{}", x, y),
        Message::Write(message) => println!("This is message: {}", message),
        Message::ChangeColor(x,y,z) => println!("That's your new color RGB: {},{},{}", x,y,z)
    }

    if let Message::Quit = &message {
        exit(1)
    }
    else{
        println!("Do nothing!");
    }

}
