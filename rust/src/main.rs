#![no_std]  // No implicit linking to the std lib.
#![cfg_attr(not(test), no_main)] // Outside tests, there is no runtime, we define our own entry point
use core::panic::PanicInfo;

#[link(wasm_import_module = "host")]
extern "C" {
    fn putchar(c: i32);
}

fn puts(str: &[u8]) {
    for c in str.iter() {
        unsafe {putchar(*c as i32)};
    }
    unsafe {putchar('\n' as i32)};
}

// writes ASCII number to buf, returns length of string.
// buf must be large enough. 11 bytes should be enough
// to hold the string representation (including sign) of an i32.
fn int_to_ascii(buf: &mut [u8], _i: i32) -> usize{
    let mut len: usize = 0;
    for c in b"TODO itoa".iter(){
        buf[len] = *c;
        len += 1;
    }
    return len
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        //TODO use std::str::from_utf8_unchecked to convert &[u8] -> &str
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    puts(b"PANIC!");
    loop{};
}

fn fizzbuzz(n: i32){
    let mut buf: [u8; 11] = [0; 11];
    if n % 15 == 0 {
        puts(b"fizzbuzz");
    } else if n % 5 == 0 {
        puts(b"buzz");
    } else if n % 3 == 0 {
        puts(b"fizz");
    } else {
        let length = int_to_ascii(&mut buf, n);
        puts(&buf[0..length]);
    }
}

#[cfg(not(test))]
#[no_mangle]
pub fn main(){
    for n in 0..100 {
        fizzbuzz(n);
    }
}