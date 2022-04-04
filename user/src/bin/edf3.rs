#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::get_time;

// const PERIOD: isize = 2000;
// const TIME: isize = 300;
const INIT: isize = 120;

#[no_mangle]
pub fn main() -> i32 {
    let start = get_time() - INIT;
    println!("current time_msec = {}, I am edf3", start);
    let mut a: usize = 1;
    let mut b: usize = 1;
    let mut c:usize = 0;
    for _i in 0..120000000{
        c = (a + b) % 1000007;
        a = b;
        b = c;    
    }
    println!("{}",c);
    let end = get_time() - INIT;
    println!(
        "time_msec = {}, delta = {}ms, edf3 complete",
        end,
        end - start
    );
    0
}