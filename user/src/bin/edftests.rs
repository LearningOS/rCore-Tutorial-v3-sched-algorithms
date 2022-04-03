#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "edf1\0",
    "edf2\0",
    "edf3\0",
];

// static TIMES: [usize;3] = [
//     800,
//     500,
//     400
// ];

static DEADLINES: [usize; 3] = [
    1000,
    2000,
    800,
];

use user_lib::{exec, fork, get_time, sleep};

#[no_mangle]
pub fn main() -> i32 {
    let mut i = 0;
    for test in TESTS {
        if i == 2{
            sleep(1000);
        }
        println!("{} Arriving at {}", test, get_time());
        let pid = fork();
        if pid == 0 {
            exec(*test, DEADLINES[i], &[core::ptr::null::<u8>()]);
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
