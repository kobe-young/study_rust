fn main() {
    println!("Hello, world!");
    never_return3();
}

fn never_return() -> ! {
    panic!("I return nothing!")
}

use std::thread;
use std::time;

fn never_return2() -> ! {
    loop {
        println!("I return nothing");
        // sleeping for 1 second to avoid exhausting the cpu resource
        thread::sleep(time::Duration::from_secs(1))
    }
}

fn never_return3() -> ! {
    todo!();
}