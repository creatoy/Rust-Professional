use std::cmp::max;

pub fn find_max_prime_factor(n: u128) -> u128 {
    if n <= 1 {
        return 0;
    }
    let factors = factor(n);
    factors.into_iter().max().unwrap_or(0)
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp >>= 1;
        base = mod_mul(base, base, modulus);
    }
    result
}

fn mod_mul(mut a: u128, mut b: u128, m: u128) -> u128 {
    // 两数相乘求模分解成多个乘法和加法操作，以避免溢出
    let mut result = 0;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    result
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // 使用确定的基集合来覆盖更大的范围，筛选出可能的质因子
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &bases {
        if a as u128 >= n {
            continue;
        }
        let a_u = a as u128;
        let n_u = n as u128;
        let d_u = d as u128;
        let mut x = mod_pow(a_u, d_u, n_u);
        if x == 1 || x == n_u - 1 {
            continue;
        }
        let mut is_composite = true;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n_u);
            if x == n_u - 1 {
                is_composite = false;
                break;
            }
        }
        if is_composite {
            return false;
        }
    }
    true
}

// Pollard-Rho 算法
fn pollards_rho(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }

    let mut c = 1;
    loop {
        let f = |x: u128| (mod_pow(x, 2, n) + c) % n;
        let mut x = 2;
        let mut y = 2;
        let mut d = 1;
        let mut trials = 0;
        while d == 1 && trials < 100_000 {
            x = f(x);
            y = f(f(y));
            d = gcd(if x > y { x - y } else { y - x }, n);
            trials += 1;
        }
        if d != 1 && d != n {
            return d;
        }
        c += 1;
        if c > 100 {
            break;
        }
    }
    n
}

fn factor(n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    let mut n = n;
    for d in [2, 3, 5] {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
    }
    if n == 1 {
        return factors;
    }
    let mut stack = vec![n];
    while let Some(current) = stack.pop() {
        if current == 1 {
            continue;
        }
        if is_prime(current) {
            factors.push(current);
            continue;
        }
        let d = pollards_rho(current);
        if d == current {
            factors.push(d);
            continue;
        }
        stack.push(d);
        stack.push(current / d);
    }
    factors
}
