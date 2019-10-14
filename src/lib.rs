use std::convert::TryFrom;
use std::{ptr, slice};

mod sanitise;

#[no_mangle]
pub unsafe extern "C" fn strip_tags(
    input: *const u8,
    input_len: i32,
    output: *mut *const u8,
    output_length: *mut i32,
) {
    let res = usize::try_from(input_len)
        .ok()
        .map(|len| slice::from_raw_parts::<u8>(input, len))
        .and_then(|slice| std::str::from_utf8(slice).ok())
        .and_then(|input_str| sanitise::strip_tags(input_str).ok())
        .and_then(|string| i32::try_from(string.len()).ok().map(|len| (string, len)));

    match res {
        Some((string, len)) => {
            let bytes = string.into_boxed_str();
            *output = bytes.as_ptr();
            *output_length = len;
            Box::into_raw(bytes);
        }
        None => {
            *output = ptr::null_mut();
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn strip_tags_free( bytes: *mut u8,) {
    Box::from_raw(bytes);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
