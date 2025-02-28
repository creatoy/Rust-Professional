use std::collections::HashSet;

pub fn goldbach_conjecture() -> String {
    let mut result = Vec::new();
    let mut count = 0;

    // 从 9 开始，逐个检查奇合数是否不满足猜想
    let mut n = 9;
    loop {
        if !is_prime(n) {
            let max_x = (((n - 2) as f32) / 2.0).sqrt().floor() as i32;
            // 遍历是否存在一个素数 p，使得 n = p + 2 * x^2，若存在，则满足了猜想(不是我们想找的值)，若找不到，则表示不满足猜想(是要找的值)
            let mut found = false;
            for x in 1..=max_x {
                let p = n - 2 * x * x;
                if p < 2 {
                    continue;
                }
                if is_prime(p) {
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(n);
                count += 1;
                if count == 2 {
                    return result
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                }
            }
        }
        n += 2;
    }
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
