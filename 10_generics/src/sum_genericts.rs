use std::convert::TryInto;

fn sum<T: TryInto<i32>>(v: Vec<T>) -> Result<i32, T::Error> {
    let mut ans = 0;
    for item in v {
        ans += item.try_into()?;
    }
    Ok(ans)
}

fn main() {
    let arr1: Vec<i8> = vec![1,2,3,4];
    let sum1 = sum(arr1).unwrap();
    println!("Total: {:?}", sum1);

    let arr2 = vec![9,8,7,7];
    let sum2 = sum(arr2).unwrap();
    println!("Total: {:?}", sum2);
}
