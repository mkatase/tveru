// src/utils/report.rs

use std::collections::{HashMap, HashSet};
use crate::ProgramGroup;

/// TVerデータの集約結果をターミナルにダンプ
pub fn dump_aggregation_report(
    raw_count: usize,
    rolled_up: &[ProgramGroup],
    unknown_stations: &HashSet<String>,
) {
    let compressed_count = rolled_up.len();

    // 圧縮率のレポート出力
    println!("====================================================");
    println!(" [DATA REPORT] TVerデータ 緩和シミュレーション");
    println!("====================================================");
    println!("  ● 総生エピソード数 (Raw):   {} 件", raw_count);
    println!("  ● 番組単位集約後 (Rolled):  {} 件", compressed_count);
    println!(
        "  ● タイムライン圧縮率:       {:.1}% 削減！",
        (1.0 - (compressed_count as f64 / raw_count as f64)) * 100.0
    );
    println!("====================================================");
    println!(
        " 🚨 [DEBUG] OTHER に分類された未登録の局一覧 (計 {} 件):",
        unknown_stations.len()
    );

    let mut sorted_unknowns: Vec<String> = unknown_stations.iter().cloned().collect();
    sorted_unknowns.sort();
    for station in sorted_unknowns {
        println!("    - \"{}\"", station);
    }
    println!("====================================================");

    // 系列グループ別の番組数カウント
    let mut stats: HashMap<String, usize> = HashMap::new();
    for program in rolled_up {
        *stats.entry(program.station_code.clone()).or_insert(0) += 1;
    }

    let display_order = vec!["NTV", "TBS", "CX", "EX", "TX", "NHK", "OTHER"];

    println!(" 📡 [SERIES REPORT] 系列グループ別 番組配信状況");
    println!("====================================================");
    for code in display_order {
        let count = stats.get(code).unwrap_or(&0);
        let label = match code {
            "NTV"   => "日本テレビ系列 (NTV) ",
            "TBS"   => "TBSテレビ系列  (TBS) ",
            "CX"    => "フジテレビ系列  (CX)  ",
            "EX"    => "テレビ朝日系列  (EX)  ",
            "TX"    => "テレビ東京系列  (TX)  ",
            "NHK"   => "NHK総合・Eテレ  (NHK) ",
            "OTHER" => "独立系・その他  (OTHER)",
            _ => "未知の系列",
        };
        let visual_bar = "■".repeat(*count / 2);
        println!("  ● {:<20} : {:>3} 番組 {}", label, count, visual_bar);
    }
    println!("====================================================");

    // リアルタイム vs 過去作アーカイブ 仕分け確認
    let mut real_count = 0;
    let mut arch_count = 0;

    println!("\n====================================================");
    println!(" 🕰️ [TIME-AXIS CHECK] リアルタイム / 過去作の仕分け確認");
    println!("====================================================");

    println!(" ［📺 今期リアルタイム現行モノ］");
    for program in rolled_up {
        if !program.is_archive {
            real_count += 1;
            if real_count <= 5 {
                println!("    - [{}] {}", program.station_code, program.title);
            }
        }
    }
    if real_count > 5 {
        println!("    ... ほか {} 件", real_count - 5);
    }

    println!("\n ［📦 過去作一挙配信・アーカイブ］");
    for program in rolled_up {
        if program.is_archive {
            arch_count += 1;
            if arch_count <= 5 {
                println!("    - [{}] {}", program.station_code, program.title);
            }
        }
    }
    if arch_count > 5 {
        println!("    ... ほか {} 件", arch_count - 5);
    }

    println!("----------------------------------------------------");
    println!("  ● リアルタイム判定: {:>3} 番組", real_count);
    println!("  ● 過去アーカイブ判定: {:>3} 番組", arch_count);
    println!("====================================================");
}
