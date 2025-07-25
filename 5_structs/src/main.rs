
fn main() {
    let a: Option<u8> = Some(5);

    if let Some(value) = a {
        println!("Value is: {}", value);
    } else {
        println!("No value found");
    }
}

// fn main () {
//     // let value: Option<i32> = Some(123);
//     let value: Option<i32> = None;

//     match value {
//         Some(v) => println!("Value is: {}", v),
//         None => println!("No value found"),
//     }
// }