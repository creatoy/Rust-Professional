pub fn retire_time(time: &str, tp: &str) -> String {
    let time = time
        .split("-")
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect::<Vec<_>>();
    assert!(time.len() == 2);

    let (year, month) = (time[0], time[1]);

    let (original_age, max_age, annual_delay, start_year) = match tp {
        "男职工" => (60, 63, 4, 2025),
        "原法定退休年龄55周岁女职工" => (55, 58, 4, 2025),
        "原法定退休年龄50周岁女职工" => (50, 55, 2, 2025),
        _ => panic!("未知人员类型: {}", tp),
    };

    let original_retire_year = year + original_age;
    let delay_months = if original_retire_year < start_year {
        0
    } else {
        let mut months = (original_retire_year - start_year) * 12 + month;
        months = months.div_ceil(annual_delay);

        months.min((max_age - original_age) * 12)
    };

    let delay_years = delay_months / 12;
    let mut retire_year = year + original_age + delay_years;
    let mut retire_month = month + delay_months % 12;
    if retire_month > 12 {
        retire_year += 1;
        retire_month -= 12;
    }

    let retire_age = original_age as f32 + delay_months as f32 / 12.0;

    let retire_age_str = if retire_age.fract() > 0.0 {
        format!("{:.2}", retire_age)
    } else {
        format!("{:.0}", retire_age)
    };

    format!("{retire_year}-{retire_month:02},{retire_age_str},{delay_months}")
}
