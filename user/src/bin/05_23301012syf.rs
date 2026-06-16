#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
let mut data = [0usize; 10];
let mut sum = 0usize;
for i in 0..10 {
data[i] = i + 1;
}
for i in 0..10 {
sum += data[i];
}
println!("addr_test sum = {}", sum);
println!("Test addr_test OK!");
0
}
