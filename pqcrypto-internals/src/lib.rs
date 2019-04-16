use rand::prelude::*;
use libc;

#[no_mangle]
pub extern "C" fn randombytes(buf: *mut libc::uint8_t, xlen: libc::size_t) -> libc::c_int {
    let mut rng = thread_rng();
    let buf = unsafe { std::slice::from_raw_parts_mut(buf, xlen as usize) };
    for x in buf.iter_mut() {
        *x = rng.gen();
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_randombytes() {
        let mut buf: [libc::uint8_t; 100] = unsafe { mem::uninitialized() };
        randombytes(buf.as_mut_ptr(), 100);
    }
}
