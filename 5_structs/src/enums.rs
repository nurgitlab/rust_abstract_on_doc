#[derive(Debug)]
struct Dog {
    name: String,
    age: u8,
    color: Color,
}

 #[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    CustomColor(String),
}
use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

fn main () {
    let my_dog = Dog {
        name: String::from("Buddy"),
        age: 5,
        color: Color::Red,
    };

    let color_red = Color::Red.to_string();
    println!("Color of my dog: {}", color_red);
    println!("{:#?}", my_dog);
}