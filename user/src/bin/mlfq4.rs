#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use::user_lib::sleep;

#[no_mangle]
pub fn main() -> i32 {
    let mut a: usize = 1;
    let mut b: usize = 1;
    let mut c:usize = 0;
    for i in 0..60000000{
        c = (a + b) % 1000007;
        a = b;
        b = c; 
        if i % 3000000 == 0{
            println!("mlfq4 running...");
            sleep(5);
        }
    }
    println!("{}",c);
    println!("mlfq4 OK");
    0
}