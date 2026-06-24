// tveru/src/main.rs

use clap::Parser;
use chrono::Datelike;
use tokio;

use tveru::{Args, BrowserConfig, ProgramGroup};
use tveru::{display, fetch, launch, reconfig, sift};
use tveru::cache;
use tveru::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut y: Vec<ProgramGroup> = Vec::new();
    let mut cache_hit = false;

    // -------------------------------------------------------------------------
    // 🛡️ キャッシュ（tmpfsのJSON）があり、かつ --refresh がなければ復元
    // -------------------------------------------------------------------------
    if !args.refresh {
        if let Some(cached_data) = cache::load() {
            y = cached_data;
            cache_hit = true;
        }
    }

    // -------------------------------------------------------------------------
    // 🚗 キャッシュがない場合のみ、構造体再編を回す
    // -------------------------------------------------------------------------
    if !cache_hit {
        // WebDriverの起動・接続
        let (_child, driver) = BrowserConfig::launch().await?;
        
        // TVerへジャンプして生のスクリプトデータをFetch
        let x = fetch::get_scripts_two(&driver, "drama", args.full).await?;
        // 🛡️ 用が済んだらブラウザは閉じる
        let _ = driver.quit().await;

        // 🗜️ 激重な構造体集約・再編処理（1度だけ実行）
        y = reconfig::aggregate_programs(x);

        // 💾 次回のために tmpfs へデータを保存
        cache::save(&y);
        // 🎯 【デバッグ用】集約された全番組のタイトルと局コードをターミナルに出力
        // for p in &y { println!("DEBUG: [{}] -> {}", p.station_code, p.title); }
    }
    // `--day` が空なら、今日の曜日を自動解決
    let target_day = args.day.clone().unwrap_or_else(|| {
        format!("{:?}", chrono::Local::now().weekday())
    });

    // =========================================================================
    // 🎨 等幅タイムライン描画（1発出力）
    // =========================================================================

    // 引数に基づいてデータを絞り込む
    let filtered_programs: Vec<_> = sift::filter(&y, &target_day, &args);

    // =========================================================================
    // 🚀 再生モード（固定されたデータから即起動）
    // =========================================================================
    launch::to_media(&filtered_programs, &args)?;

    if args.index.is_some() && args.episode.is_some() {
        return Ok(());
    }

    // =========================================================================
    // 🎨 息抜き特化型・等幅タイムライン描画（1発出力）
    // =========================================================================
    display::to_terminal(&filtered_programs, &target_day, &args)?;

    Ok(())
}
