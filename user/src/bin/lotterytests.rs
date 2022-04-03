#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "lottery0\0",
    "lottery1\0",
    "lottery2\0",
    "lottery3\0",
    "lottery4\0",
    "lottery5\0",
];


use user_lib::{exec, fork, get_time, sleep};

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {     
        let start = get_time();
        println!("{} Arrive at {}", test, start);
        let pid = fork();
        if pid == 0 {
            exec(*test, &[core::ptr::null::<u8>()]);
            panic!("unreachable!");
        }
    }
    sleep(15000);
    for test in TESTS {     
        let start = get_time();
        println!("{} Arrive at {}", test, start);
        let pid = fork();
        if pid == 0 {
            exec(*test, &[core::ptr::null::<u8>()]);
            panic!("unreachable!");
        }
    }
    0
}
