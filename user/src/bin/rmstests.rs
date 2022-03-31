#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "rms1\0",
    "rms2\0",
];

// static TIMES: [usize;2] = [
//     800,
//     500,
// ];

static PERIODS: [usize; 2] = [
    2000,
    1000,
];

use user_lib::{exec, fork, get_time, sleep};

#[no_mangle]
pub fn main() -> i32 {
    let mut i = 0;
    for test in TESTS {
        println!("{} Arriving at {}", test, get_time());
        let pid = fork();
        if pid == 0 {
            exec(*test, PERIODS[i], &[core::ptr::null::<u8>()]);
            panic!("unreachable!");
        }
        i += 1;
    }

        // for i in 0..2{
        //     if j % PERIODS[i] == 0{
        //         let test = TESTS[i];
        //         println!("{} Arriving at {}", test, get_time());
        //         let pid = fork();
        //         if pid == 0 {
        //             exec(test, TIMES[i], &[core::ptr::null::<u8>()]);
        //             panic!("unreachable!");
        //         }
        //     }
        // }
        // sleep(500);
    0
}
