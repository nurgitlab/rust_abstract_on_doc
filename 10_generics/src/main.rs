// fn sum_i8 (v: Vec<i8>) -> i32 {
//     let mut ans = 0;

//     for i in 0..v.len() {
//         ans+=v[i] as i32;
//     }

//     return ans;
// }

// fn sum_i32 (v: Vec<i32>) -> i32 {
//     let mut ans: i32 = 0;

//     for i in 0..v.len() {
//         ans+=v[i] as i32;
//     }

//     return ans;
// }

//=======

// #[derive(Debug)] //need to write in console
// struct User<T,U> {
//     balance: T,
//     age: T,
//     tax: U,
// }


// fn main() {
//     let user = User { 
//         balance: 10, 
//         age: 10, 
//         tax: 1.0
//     };
//     println!("User {:#?}", user);
//     println!("This is a shared function from main.rs");
// }
