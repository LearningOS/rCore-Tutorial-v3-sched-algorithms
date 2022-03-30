#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "sjf1\0",
    "sjf2\0",
    "sjf3\0",
    "sjf4\0",
];

static TIMES: [usize;4] = [
    10000,
    100000,
    1000,
    100,
];

use user_lib::{exec, fork, sleep};

#[no_mangle]
pub fn main() -> i32 {
    let mut i = 0;
    for test in TESTS {     
        if(i == 3){
            sleep(1000);
        }
        println!("Usertests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            exec(*test, TIMES[i], &[core::ptr::null::<u8>()]);
            panic!("unreachable!");
        }
        i += 1;
    }
    println!("Usertests passed!");
    0
}
