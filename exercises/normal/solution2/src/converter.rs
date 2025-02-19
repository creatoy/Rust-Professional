use std::ops::RemAssign;

fn str_to_int(num: &str, base: u32) -> i32 {
    let mut result = 0;
    let mut power = 1;

    for c in num.chars().rev() {
        if base > 10 && c >= 'a' && c <= 'f' {
            result += (c as i32 - 'a' as i32 + 10) * power;
        } else {
            result += (c as i32 - '0' as i32) * power;
        }
        power *= base as i32;
    }

    result
}

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let num = num_str.trim_end_matches(')').split('(').collect::<Vec<_>>();
    if num.len() != 2 {
        return "".into();
    }

    let base = num[1].parse::<u32>().unwrap();
    if base < 2 || base > 16 {
        return "".into();
    }

    let mut val = str_to_int(&num[0], base);

    let mut result = vec![];

    while val != 0 {
        let rem = val % to_base as i32;
        let rem = if rem < 10 {
            rem + '0' as i32
        } else {
            rem - 10 + 'a' as i32
        };
        result.push(rem as u8 as char);
        val /= to_base as i32;
    }

    result.reverse();

    result.iter().collect()
}
