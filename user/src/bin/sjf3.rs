#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{sleep,get_time};

#[no_mangle]
pub fn main() -> i32 {
    let start = get_time();
    print!("I am sjf3");
    println!("current time_msec = {}", start);
    let mut a: usize = 1;
    let mut b: usize = 1;
    let mut c:usize = 0;
    for i in (0..500000000){
        c = (a + b) % 1000007;
        a = b;
        b = c;    
    }
    println!("{}",c);
    let end = get_time();
    println!(
        "time_msec = {} after sleeping 100 ticks, delta = {}ms!",
        end,
        end - start
    );
    0
}