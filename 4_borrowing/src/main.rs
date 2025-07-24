//Example 3
fn main() {

    let s1 = String::from("Hello, world!");
    let s2 = &s1[0..5]; //Slice of s1

    println!("s1: {}, s2: {}", s1, s2);
}
//Example 2
// fn main() {
//     let mut s1 = String::from("Hello, world!");
//     let l = str_len(&s1);
//     add_to_str(&mut s1, " How are you?");
//     println!("s1: {}, l: {}", s1, l);
// }

// fn str_len (s: &String)  -> usize {
//     s.len()
// }

// fn add_to_str (s: &mut String, t: &str) {
//     s.push_str(t);
// }



//Example 1
// fn main() {
//     let s1 := String::from("Hello, world!");
//     let l = str_len(s1);
//     println!("s1: {}", s1); Error: value moved, cannot be used again
// }

// fn str_len (s: String)  -> usize {
//     s.len()
// }
