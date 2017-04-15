fn is_leap_year(year: usize) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

// Months are indexed so that January is 1, February is 2, etc.
fn month_days(month: usize, year: usize) -> usize {
    // Indexed by month. Start with a dummy zero so January is 1.
    let days = vec![
        0,
        31, // January
        28, // February
        31, // March
        30, // April
        31, // May
        30, // June
        31, // July
        31, // August
        30, // September
        31, // October
        30, // November
        31, // December
    ]; 
    if (month != 2) || !is_leap_year(year) {days[month]} else {29}
}

pub fn solve() -> usize {
    // Starting on a Monday on 1900-01-01.
    let mut weekday = 1;

    // Get weekday for 1901-01-01.
    for month in 1..13 {
        weekday += month_days(month, 1900);
        weekday %= 7;
    }

    // Check all months in the twentieth century.
    let mut count = 0;
    for year in 1901..2001 {
        for month in 1..13 {
            if weekday == 0 {count += 1;}
            weekday += month_days(month, year);
            weekday %= 7;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(171, solve());
    }
}
