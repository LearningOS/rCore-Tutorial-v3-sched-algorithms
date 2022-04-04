#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{get_time, cycle};

const PERIOD: isize = 1500;
const INIT: isize = 3120;

#[no_mangle]
pub fn main() -> i32 {
    for j in 0..4{
        let start = get_time() - INIT;
        println!("current time_msec = {}, I am rms1", start);
        let mut a: usize = 1;
        let mut b: usize = 1;
        let mut c:usize = 0;
        for _i in 0..320000000{
            c = (a + b) % 1000007;
            a = b;
            b = c;    
        }
        println!("{}",c);
        let end = get_time() - INIT;
        println!(
            "time_msec = {}, delta = {}ms, rms1 complete",
            end,
            end - start
        );
        let sleep_time = PERIOD * (j + 1) - end;
        if sleep_time < -50{
            panic!("rms1 outlimit!")
        }
        //sleep(sleep_time as usize);
        cycle(PERIOD as usize);
    }
    0
}
