mod generator;

fn main() {
    let a: u8 = generator::generate();
    println!("{}", a);
}

pub fn shared () {
    println!("This is a shared function from main.rs");
}