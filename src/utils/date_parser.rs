// tveru/src/utils/date_parser.rs

use jiff::{civil::Date, ZonedDateTime};
use regex::Regex;

pub fn parse_broadcast_date(bdate_str: &str) -> Date {
    let today = ZonedDateTime::now().date();
    let current_year = today.year();

    // 文字列の中に「4桁の数字 + 年」が入っていれば捕獲
    let re_year_only = Regex::new(r"(\d{4})年").unwrap();
    if let Some(caps) = re_year_only.captures(bdate_str) {
        if let Ok(year) = caps[1].parse::<i16>() {
            // もしパースした年が今年より前なら、一挙配信過去作なので1月1日へ！
            if year < current_year {
                return Date::new(year, 1, 1).unwrap();
                //return NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            }
        }
    }

    // 「06月12日(金)放送分」のような今期モノ
    let re_month_day = Regex::new(r"(\d{1,2})月(\d{1,2})日").unwrap();
    if let Some(caps) = re_month_day.captures(bdate_str) {
        let month = caps[1].parse::<i8>().unwrap_or(1);
        let day = caps[2].parse::<i8>().unwrap_or(1);
       
        return Date::new(current_year, month, day).unwrap_or(today);
    }

    today
}
