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
    for i in 1..100000{
        print!(" ");
    }
    let end = get_time();
    println!(
        "time_msec = {} after sleeping 100 ticks, delta = {}ms!",
        end,
        end - start
    );
    0
}