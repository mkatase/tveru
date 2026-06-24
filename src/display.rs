// tveru/src/display.rs

use colored::*;

use crate::{Args, ProgramGroup};
use crate::constants;
use crate::utils::helper;
use crate::error::Result;

pub fn to_terminal(programs: &[ProgramGroup], target_day: &str, args: &Args) -> Result<()> {
    println!("\n------------------------------------------------------------------------");
    println!(" 📺 TVer CUI Timeline [ 曜日: {} | 局: {:?} ]", target_day, args.network);
    println!("------------------------------------------------------------------------");

    if programs.is_empty() {
        println!("  指定された条件に一致する番組は配信されていません。");
    } else {
        for (idx, p) in programs.iter().enumerate() {
            let title_width = p.title.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum::<usize>();
            let target_pad = 40; 
            let padding_spaces = if target_pad > title_width {
                " ".repeat(target_pad - title_width)
            } else {
                String::new()
            };

            let has_main_episode = p.episodes.iter().any(|ep| !ep.is_badge);

            if args.strict && !has_main_episode { continue; }

            let mut ep_nums: Vec<_> = 
                p.episodes.iter().map(|ep| (ep.episode_num, ep.is_last, ep.is_badge)).collect();
            ep_nums.sort_by_key(|(num, _, _)| *num);
            ep_nums.dedup_by_key(|(num, _, _)| *num);

            let ep_labels: Vec<String> = ep_nums.into_iter()
                .map(|(num, is_last, is_badge)| {
                    if is_last { format!("#{} [終]", num) } else {
                        if is_badge {
                            let (r, g, b) = helper::hex_to_rgb(constants::NO_DISP);
                            //format!("{:<7} | {}", index_str, truncated_date).truecolor(r, g, b).to_string()
                            format!("#{}", num).truecolor(r, g, b).to_string()
                        } else {
                            format!("#{}", num)
                        }
                    }
                })
                .collect();
            let ep_string = ep_labels.join(" ");
            let end_label = p.episodes.first().map(|ep| ep.end_label.as_str()).unwrap_or("");

            println!(
                "[{:>3}] | {:<5} | {}{} | {:<12} | {}",
                idx, p.station_code, p.title, padding_spaces, ep_string, end_label
            );
        }
    }
    println!("------------------------------------------------------------------------\n");

    Ok(())
}
