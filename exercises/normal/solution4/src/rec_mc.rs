pub fn dp_rec_mc(amount: u32) -> u32 {
    const CASHE: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];

    // 贪心方法，依次使用最大面值找零
    // let mut count = 0;
    // let mut amount = amount;
    // CASHE.iter().for_each(|&coin| {
    //     if coin <= amount {
    //         count += 1;
    //         amount -= coin;
    //     }
    // });
    // count

    // 动态规划
    let mut dp = vec![u32::MAX; amount as usize + 1];
    dp[0] = 0;
    for i in 1..=amount as usize {
        for coin in &CASHE {
            if *coin as usize <= i {
                dp[i] = dp[i].min(dp[i - *coin as usize] + 1);
            }
        }
    }

    dp[amount as usize]
}
