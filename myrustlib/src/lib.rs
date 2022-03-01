use leftpad::left_pad;
use std::io::{self,Write};

#[no_mangle]
pub extern "C" fn run(n: usize) {
    println!("{}, world", left_pad("hello", n));
    io::stdout().flush().unwrap();
}
