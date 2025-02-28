use std::{cmp::Ordering, fmt::format, str::FromStr};

// const EXCHANGE_CLOSE_DATE: [(Date, Date); 1] = [(Date::new(2024, 1, 1), Date::new(2024, 1, 1))];

pub fn time_info(time: &str) -> String {
    let date = Date::from_str(time).unwrap();

    println!("{:?}", date);

    let week = date.iso_week();
    let weekday = date.weekday();
    let day = date.day_of_year();
    let last = date.days_in_year() - day;
    let days_to_cny = date.days_to_chinese_new_year();
    let days_to_open = date.days_to_exchange_open();

    format!("{week},{weekday},{day},{last},{days_to_cny},{days_to_open}")
}

#[derive(Debug, Clone, Copy)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    fn new(year: u32, month: u32, day: u32) -> Self {
        Date { year, month, day }
    }

    fn query_chinese_new_year(year: u32) -> Date {
        match year {
            2024 => Date::new(2024, 2, 10),
            2025 => Date::new(2025, 1, 29),
            2026 => Date::new(2026, 2, 17),
            _ => unimplemented!("Not implemented"),
        }
    }

    fn query_exchange_close_date() -> Vec<(Date, Date)> {
        vec![
            (Date::new(2025, 1, 1), Date::new(2025, 1, 1)),
            (Date::new(2025, 1, 28), Date::new(2025, 2, 4)),
            (Date::new(2025, 4, 4), Date::new(2025, 4, 6)),
            (Date::new(2025, 5, 1), Date::new(2025, 5, 5)),
            (Date::new(2025, 5, 31), Date::new(2025, 6, 2)),
            (Date::new(2025, 10, 1), Date::new(2025, 10, 8)),
            (Date::new(2026, 1, 1), Date::new(2026, 1, 1)),
        ]
    }

    fn is_leap_year(&self) -> bool {
        if self.year % 4 != 0 {
            return false;
        } else if self.year % 100 != 0 {
            return true;
        } else if self.year % 400 != 0 {
            return false;
        } else {
            return true;
        }
    }

    fn days_in_month(&self) -> u32 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    fn days_in_year(&self) -> u32 {
        if self.is_leap_year() {
            366
        } else {
            365
        }
    }

    fn day_of_year(&self) -> u32 {
        let mut days = 0;
        if self.month > 2 {
            days += if self.is_leap_year() { 1 } else { 0 };
        }
        for m in 1..self.month {
            days += Self::DAYS_IN_MONTH[(m - 1) as usize];
        }
        days += self.day;
        days
    }

    fn days_to_chinese_new_year(&self) -> u32 {
        let date_of_cny = if self < &Date::query_chinese_new_year(self.year) {
            Date::query_chinese_new_year(self.year)
        } else {
            Date::query_chinese_new_year(self.year + 1)
        };
        self.days_between_date(&date_of_cny)
    }

    fn days_to_exchange_open(&self) -> u32 {
        let date = self.date_after_days(1);
        Date::query_exchange_close_date()
            .iter()
            .find(|(start, end)| {
                if &date >= start && &date <= end {
                    true
                } else {
                    false
                }
            })
            .map(|(_, end)| date.days_between_date(end) + 1)
            .unwrap_or(if date.weekday() < 6 {
                0
            } else {
                8 - date.weekday()
            })
    }

    // 计算当前日期是星期几（基姆拉尔森公式）
    fn weekday(&self) -> u32 {
        let m = self.month;
        let d = self.day;
        let mut y = self.year;
        let mut m = m;
        if m < 3 {
            m += 12;
            y -= 1;
        }

        let h = (d + 13 * (m + 1) / 5 + y + y / 4 - y / 100 + y / 400) % 7;

        (h + 5) % 7 + 1
    }

    fn iso_week(&self) -> u32 {
        let weekday = self.weekday();
        let date = if weekday < 4 {
            self.date_after_days(4 - weekday)
        } else {
            self.date_before_days(weekday - 4)
        };
        let day = date.day_of_year();
        let week = day / 7 + 1;

        week
    }

    fn days_to_next_month(&self) -> u32 {
        self.days_in_month() - self.day + 1
    }

    fn days_between_date(&self, other: &Date) -> u32 {
        let mut days = 0;
        let (mut start, end) = if self < other {
            (self.clone(), other)
        } else {
            (other.clone(), self)
        };

        if start.year == end.year {
            days += end.day_of_year() - start.day_of_year();
        } else {
            days += start.days_in_year() - start.day_of_year();
            start.year += 1;

            while start.year < end.year {
                days += start.days_in_year();
                start.year += 1;
            }
            days += end.day_of_year();
        }

        days
    }

    fn date_after_days(&self, days: u32) -> Date {
        let mut date = Date::new(self.year, self.month, self.day);
        let mut days = days;
        if days < self.days_to_next_month() {
            date.day += days;
        } else {
            days -= self.days_to_next_month();
            date.month += 1;
            if date.month > 12 {
                date.month = 1;
                date.year += 1;
            }

            while days > date.days_in_month() {
                days -= date.days_in_month();
                date.month += 1;
                if date.month > 12 {
                    date.month = 1;
                    date.year += 1;
                }
            }

            date.day = days + 1;
        }

        date
    }

    fn date_before_days(&self, days: u32) -> Date {
        let mut date = Date::new(self.year, self.month, self.day);
        let mut days = days;
        if days < self.day {
            date.day -= days;
        } else {
            days -= self.day;
            date.month -= 1;
            if date.month == 0 {
                date.month = 12;
                date.year -= 1;
            }

            while days > date.days_in_month() {
                days -= date.days_in_month();
                date.month -= 1;
                if date.month == 0 {
                    date.month = 12;
                    date.year -= 1;
                }
            }

            date.day = date.days_in_month() - days;
        }

        date
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('-').collect();
        if parts.len() != 3 {
            return Err("Invalid date format".into());
        }
        let year = parts[0].parse::<u32>().map_err(|_| "Invalid year")?;
        let month = parts[1].parse::<u32>().map_err(|_| "Invalid month")?;
        let day = parts[2].parse::<u32>().map_err(|_| "Invalid day")?;
        Ok(Date::new(year, month, day))
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year)? {
            Ordering::Equal => match self.month.partial_cmp(&other.month)? {
                Ordering::Equal => self.day.partial_cmp(&other.day),
                other => Some(other),
            },
            other => Some(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Date;

    #[test]
    fn test_days_in_month() {
        let date1 = Date::new(2024, 2, 3);
        let date2 = Date::new(2025, 2, 1);
        let date3 = Date::new(2025, 5, 5);
        assert_eq!(date1.days_in_month(), 29);
        assert_eq!(date2.days_in_month(), 28);
        assert_eq!(date3.days_in_month(), 31);
    }

    #[test]
    fn test_days_to_next_month() {
        let date1 = Date::new(2024, 2, 3);
        let date2 = Date::new(2025, 2, 1);
        let date3 = Date::new(2025, 5, 5);
        assert_eq!(date1.days_to_next_month(), 27);
        assert_eq!(date2.days_to_next_month(), 28);
        assert_eq!(date3.days_to_next_month(), 27);
    }

    #[test]
    fn test_days_of_year() {
        let date1 = Date::new(2024, 3, 3);
        let date2 = Date::new(2025, 1, 15);
        let date3 = Date::new(2025, 5, 5);
        let date4 = Date::new(2025, 12, 31);
        assert_eq!(date1.day_of_year(), 63);
        assert_eq!(date2.day_of_year(), 15);
        assert_eq!(date3.day_of_year(), 125);
        assert_eq!(date4.day_of_year(), 365);
    }

    #[test]
    fn test_weekday() {
        let date1 = Date::new(2025, 1, 1);
        let date2 = Date::new(2025, 1, 18);
        let date3 = Date::new(2025, 12, 31);
        let date4 = Date::new(2025, 11, 1);
        let date5 = Date::new(2025, 2, 28);
        assert_eq!(date1.weekday(), 3);
        assert_eq!(date2.weekday(), 6);
        assert_eq!(date3.weekday(), 3);
        assert_eq!(date4.weekday(), 6);
        assert_eq!(date5.weekday(), 5);
    }

    #[test]
    fn test_iso_week() {
        let date1 = Date::new(2025, 1, 1);
        let date2 = Date::new(2025, 1, 18);
        let date3 = Date::new(2025, 12, 31);
        let date4 = Date::new(2025, 11, 1);
        let date5 = Date::new(2025, 2, 28);
        assert_eq!(date1.iso_week(), 1);
        assert_eq!(date2.iso_week(), 3);
        assert_eq!(date3.iso_week(), 1);
        assert_eq!(date4.iso_week(), 44);
        assert_eq!(date5.iso_week(), 9);
    }

    #[test]
    fn test_days_to_chinese_new_year() {
        let date1 = Date::new(2025, 1, 1);
        let date2 = Date::new(2025, 1, 18);
        let date3 = Date::new(2025, 12, 31);
        let date4 = Date::new(2025, 11, 1);
        let date5 = Date::new(2025, 2, 28);
        assert_eq!(date1.days_to_chinese_new_year(), 28);
        assert_eq!(date2.days_to_chinese_new_year(), 11);
        assert_eq!(date3.days_to_chinese_new_year(), 48);
        assert_eq!(date4.days_to_chinese_new_year(), 108);
        assert_eq!(date5.days_to_chinese_new_year(), 354);
    }

    #[test]
    fn test_days_to_exchange_open() {
        let date1 = Date::new(2025, 1, 1);
        let date2 = Date::new(2025, 1, 18);
        let date3 = Date::new(2025, 12, 31);
        let date4 = Date::new(2025, 11, 1);
        let date5 = Date::new(2025, 2, 28);
        assert_eq!(date1.days_to_exchange_open(), 0);
        assert_eq!(date2.days_to_exchange_open(), 1);
        assert_eq!(date3.days_to_exchange_open(), 1);
        assert_eq!(date4.days_to_exchange_open(), 1);
        assert_eq!(date5.days_to_exchange_open(), 2);
    }

    #[test]
    fn test_date_after_days() {
        let date1 = Date::new(2024, 3, 3);
        let date2 = Date::new(2025, 1, 15);
        let date3 = Date::new(2025, 5, 5);
        let date4 = Date::new(2025, 12, 31);
        assert_eq!(date1.date_after_days(10), Date::new(2024, 3, 13));
        assert_eq!(date2.date_after_days(365), Date::new(2026, 1, 15));
        assert_eq!(date3.date_after_days(33), Date::new(2025, 6, 7));
        assert_eq!(date4.date_after_days(10), Date::new(2026, 1, 10));
    }

    #[test]
    fn test_date_before_days() {
        let date1 = Date::new(2024, 3, 13);
        let date2 = Date::new(2026, 1, 15);
        let date3 = Date::new(2025, 6, 7);
        let date4 = Date::new(2026, 1, 10);
        assert_eq!(date1.date_before_days(10), Date::new(2024, 3, 3));
        assert_eq!(date2.date_before_days(365), Date::new(2025, 1, 15));
        assert_eq!(date3.date_before_days(33), Date::new(2025, 5, 5));
        assert_eq!(date4.date_before_days(10), Date::new(2025, 12, 31));
    }
}
