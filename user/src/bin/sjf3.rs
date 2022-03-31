#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{get_time};

#[no_mangle]
pub fn main() -> i32 {
    let start = get_time();
    println!("current time_msec = {}, I am sjf3", start);
    let mut a: usize = 1;
    let mut b: usize = 1;
    let mut c:usize = 0;
    for _i in 0..400000000{
        c = (a + b) % 1000007;
        a = b;
        b = c;    
    }
    println!("{}",c);
    let end = get_time();
    println!(
        "time_msec = {}, delta = {}ms, sjf3 complete",
        end,
        end - start
    );
    0
}