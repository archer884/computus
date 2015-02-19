#![feature(env)]

use std::num::Float; // needed *only* for `.floor()`

fn main() {
    let year = match std::env::args().nth(1) {
        Some(input) => match input.parse() {
            Ok(n) => n,
            _ => {
                println!("Please enter a valid year.");
                return;
            },
        },
        _ => {
            println!("Usage: ./computus YEAR");
            return;
        },
    };

    let (month, day) = get_easter_date(year);

    println!("Easter Sunday will be on {} {} in {}.",
        if month == 3 { "March" } else { "April" },
        day,
        year);
}

// This function is f-cking ridiculous. No human should ever have
// to write anything like this--much less actually try to read it.
fn get_easter_date(year: usize) -> (usize, usize) {
    let a = year % 19;
    let b = divide_with_floor(year as f32, 100f32);
    let c = year % 100;
    let d = divide_with_floor(b as f32, 4f32);
    let e = b % 4;
    let f = divide_with_floor((b + 8) as f32, 25f32);
    let g = divide_with_floor((b - f + 1) as f32, 3f32);
    let h = ((a * 19) + b - d - g + 15) % 30;
    let i = divide_with_floor(c as f32, 4f32);
    let k = c % 4;
    let l = (32 + (2 * e) + (2 * i) - h - k) % 7;
    let m = divide_with_floor((a + (11 * h) + (22 * l)) as f32, 451f32);

    (divide_with_floor((h + l - (7 * m) + 114) as f32, 31f32), ((h + l - (7 * m) + 114) % 31) + 1)
}

// I felt like this was easier than writing (a as f32 / b as f32).floor()
// every damn time, but it honestly doesn't add much value, I know. I'm
// curious what impact (if any) the inline hint will have on the code
// generated for this function.
#[inline]
fn divide_with_floor(a: f32, b: f32) -> usize {
    (a as f32 / b).floor() as usize
}

// I thought it was really freaking cool that I could build a set
// of tests onto a binary like this. I'm kinda used to that kind
// of thing being a separate project or something.
#[cfg(test)]
mod computus_tests {
    #[test]
    fn easter_2015_is_correct() {
        assert!(super::get_easter_date(2015) == (4, 5));
    }

    #[test]
    fn easter_2016_is_correct() {
        assert!(super::get_easter_date(2016) == (3, 27));
    }

    #[test]
    fn easter_2017_is_correct() {
        assert!(super::get_easter_date(2017) == (4, 16));
    }
}
