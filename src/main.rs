use buffer::SmallFixedBuffer;
use datetime::DateTimeBuffer;

pub mod datetime;
mod opt;
mod util;
mod buffer;

macro_rules! write_double_digit {
    ($buf:ident, $value:expr) => {
        if $value < 10 {
            $buf.push(b'0');
        }
        $buf.extend_from_slice(itoa::Buffer::new().format($value).as_bytes());
    };
}

macro_rules! str_from_slice {
    ($ptr:expr, $size:expr) => {
        unsafe { std::str::from_utf8_unchecked(core::slice::from_raw_parts($ptr, $size as usize)) }
    };
}

fn main() {
    let year: i32 = 2024;
    let month: u8 = 12;
    let day: u8 = 31;

    let mut dtb = DateTimeBuffer::new();
    
    let mut yearbuf = itoa::Buffer::new();
    let formatted = yearbuf.format(year);
    
    dtb.extend_from_slice(formatted.as_bytes());
    dtb.push(b'-');
    write_double_digit!(dtb, month);
    dtb.push(b'-');
    write_double_digit!(dtb, day);


    let mut sfb  = SmallFixedBuffer::new();

    sfb.extend_from_slice(formatted.as_bytes());
    sfb.push(b'-');
    write_double_digit!(sfb, month as u32);
    sfb.push(b'-');
    write_double_digit!(sfb, day);

    println!("{}", str_from_slice!(dtb.as_ptr(), dtb.len()));
    println!("{}", str_from_slice!(sfb.as_ptr(), sfb.len()));
    

}
