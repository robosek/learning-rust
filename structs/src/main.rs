#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn create(width: i32, height:i32) -> Rectangle {
        Rectangle{
            width,
            height
        }
    }
}

fn main() {

    let rectangle = Rectangle::create(123,432);
    let result = rectangle.area();

    println!("The area of rectangle {:?} is {}", rectangle, result)
}
