#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("23301012syf  app!");
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
    }

    println!("sum is {}", sum);
    0
}
