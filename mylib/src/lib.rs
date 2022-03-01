use leftpad::left_pad;

#[no_mangle]
pub extern "C" fn run(n: usize) {
    println!("{}foo", left_pad("hello", n));
}
