//! # date_diff_lib
//!
//! A library for calculating day diff in days.
//! Just for learning Rust programming.

/// Calculates date diff in days
/// Below is a test embedded inside function doc comments
/// # Examples
///
/// ```
/// assert_eq!(date_diff_lib::daydiff(2019, 5, 5, 2019, 3, 5), -61);
///
/// ```
pub fn daydiff(
    year_one: i32,
    month_one: i32,
    days_one: i32,
    year_two: i32,
    month_two: i32,
    days_two: i32,
) -> i32 {
    //this is a crazy formula, but efficient way of calculating day diff.
    //Be aware, that the calendar has changed many times in history.
    //The last change for "our" calendar was around 1753. 
    //So calculating anything before that is mathematically incorrect.
    //https://www.timeanddate.com/calendar/julian-gregorian-switch.html
    (days_two - 32075
        + 1461 * (year_two + 4800 + (month_two - 14) / 12) / 4
        + 367 * (month_two - 2 - (month_two - 14) / 12 * 12) / 12
        - 3 * (((year_two + 4900 + (month_two - 14) / 12) / 100) / 4)
        - (days_one - 32075
            + 1461 * (year_one + 4800 + (month_one - 14) / 12) / 4
            + 367 * (month_one - 2 - (month_one - 14) / 12 * 12) / 12
            - 3 * ((year_one + 4900 + (month_one - 14) / 12) / 100) / 4))
}

//region: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datediff2019050520190305() {
        //test for a random interval
        assert_eq!(daydiff(2019, 5, 5, 2019, 3, 5), -61);
    }
    #[test]
    fn datediff2017022520190305() {
        //test for a random interval
        assert_eq!(daydiff(2017, 2, 25, 2019, 5, 5), 799);
    }
    #[test]
    fn loop_from_1900_to_2100() {
        //test if it is correct for a long period of 200 years.
        let mut day_counter = 36524;
        for year_one in 1900..=2100 {
            for month_one in 1..=12 {
                for day_one in 1..=31 {
                    let mut valid_date = true;
                    if day_one == 31 && [2, 4, 6, 9, 11].contains(&month_one) {
                        valid_date = false;
                    }
                    if day_one == 30 && month_one == 2 {
                        valid_date = false;
                    }
                    if day_one == 29
                        && month_one == 2
                        && !(year_one % 4 == 0 && (year_one % 100 != 0 || year_one % 400 == 0))
                    {
                        valid_date = false;
                    }
                    if valid_date == true {
                        let daydiff = daydiff(year_one, month_one, day_one, 2000, 1, 1);
                        println!(
                            "{}  {} {} {} {}",
                            daydiff, day_counter, year_one, month_one, day_one
                        );

                        if day_counter != daydiff {
                            panic!("error: day_counter != daydiff")
                        }

                        day_counter -= 1;
                    }
                }
            }
        }
    }
}
//endregion
