pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;

    let mut sum = 2;
    let mut tmp = 0;
    while tmp < threshold {
        if tmp % 2 == 1 {
            sum += tmp;
        }

        tmp = a + b;
        a = b;
        b = tmp;
    }

    sum
}
