// tveru/src/utils/date_parser.rs

use chrono::{NaiveDate, Datelike, Local};
use regex::Regex;

pub fn parse_broadcast_date(bdate_str: &str) -> NaiveDate {
    let current_year = Local::now().year();

    // 文字列の中に「4桁の数字 + 年」が入っていれば捕獲
    let re_year_only = Regex::new(r"(\d{4})年").unwrap();
    if let Some(caps) = re_year_only.captures(bdate_str) {
        if let Ok(year) = caps[1].parse::<i32>() {
            // もしパースした年が今年より前なら、一挙配信過去作なので1月1日へ！
            if year < current_year {
                return NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            }
        }
    }

    // 「06月12日(金)放送分」のような今期モノ
    let re_month_day = Regex::new(r"(\d{1,2})月(\d{1,2})日").unwrap();
    if let Some(caps) = re_month_day.captures(bdate_str) {
        let month = caps[1].parse::<u32>().unwrap_or(1);
        let day = caps[2].parse::<u32>().unwrap_or(1);
        
        return NaiveDate::from_ymd_opt(current_year, month, day)
            .unwrap_or_else(|| Local::now().date_naive());
    }

    Local::now().date_naive()
}
