// tveru/src/sift.rs

use chrono::Datelike;

use crate::{Args, ProgramGroup};

/// 引数に基づいてデータを絞る
pub fn filter(y: &[ProgramGroup], target_day: &str,args: &Args) -> Vec<ProgramGroup> {
    y.iter()
        // アーカイブフラグの一致
        .filter(|p| p.is_archive == args.archive)
        // 局指定が含まれているか
        .filter(|p| args.network.is_empty() || args.network.contains(&p.station_code))
        // 曜日指定（今日、または指定曜日）の一致
        .filter(|p| {
            if args.archive && p.is_archive {
                return true
            }
            p.episodes.iter().any(|ep| {
                format!("{:?}", ep.broadcast_date.weekday()) == target_day
            })
        })
        .cloned()
        .collect()
}
