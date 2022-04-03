#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "mlfq1\0",
    "mlfq2\0",
    "mlfq3\0",
    "mlfq4\0",
    "mlfq5\0",
];


use user_lib::{exec, fork, sleep, get_time};

#[no_mangle]
pub fn main() -> i32 {
    let mut i = 0;
    for test in TESTS {     
        if i == 3 || i == 4{
            sleep(200);
        }
        let start = get_time();
        println!("{} Arrive at {}", test, start);
        let pid = fork();
        if pid == 0 {
            exec(*test, &[core::ptr::null::<u8>()]);
            panic!("unreachable!");
        }
        i += 1;
    }
    0
}