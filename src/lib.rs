// src/lib.rs

use clap::Parser;
use thirtyfour::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use std::process::{Child, Command, Stdio};

// -------------------------------------------------------------------------
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// 曜日を指定するフラグ (Sun, Mon, Tue, Wed, Thu, Fri, Sat) (defualt is today.)
    #[arg(short, long)]
    pub day: Option<String>,
    /// フィルタリングする系列局コード (NTV, TBS, CX, EX, TX, NHK) (ex. -n NTV,TBS)
    #[arg(short, long, value_delimiter=',')]
    pub network: Vec<String>,
    /// 過去作アーカイブを表示するフラグ
    #[arg(short, long, default_value_t = false)]
    pub archive: bool,
    /// 巡回を実行するフラグ
    #[arg(short, long, default_value_t = false)]
    pub full: bool,
    /// 非表示（本編以外）フラッグ
    #[arg(short, long, default_value_t = false)]
    pub strict: bool,
    /// 使用するメディアプレイヤー (デフォルト: mpv)
    /// ※ 環境変数 TVERU_PLAYER があればそちらが最優先されます
    #[arg(short, long, default_value = "mpv")]
    pub player: String,

    /// 最新データを再取得するフラグ
    #[arg(short, long, default_value_t = false)]
    pub refresh: bool,
    /// 再生したい番組の左端のインデックス番号 (例: 0)
    #[arg(value_name = "INDEX", value_parser = clap::value_parser!(usize))]
    pub index: Option<usize>,

    /// 再生したいエピソードの話数（数字のみ、例: 3）
    #[arg(value_name = "EPISODE", value_parser = clap::value_parser!(u32))]
    pub episode: Option<u32>,
}
// -------------------------------------------------------------------------
pub struct BrowserConfig;

impl BrowserConfig {
    pub fn create_capabilities() -> Capabilities {
        let mut caps = DesiredCapabilities::chrome();
        let _ = caps.add_arg("--headless").unwrap();
        caps.into()
    }
    // chromedriverのプロセスとWebDriverをセットで返す
    pub async fn launch() -> Result<(Child, WebDriver), WebDriverError> {
        // 既存のプロセスをキル
        let _ = Command::new("fuser")
            .args(["-k", "9515/tcp"])
            .output();

        // chromedriver を起動
        let child = Command::new("chromedriver")
            .arg("--port=9515")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            //.map_err(|e| WebDriverError::CustomError(e.to_string()))?;
            .map_err(WebDriverError::IoError)?;

        // 待機
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // WebDriverに接続
        let caps = Self::create_capabilities();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;

        Ok((child, driver))
    }
}

// -------------------------------------------------------------------------
pub struct Element {
    pub genre: String,
    pub title: String,
    pub episode: String,
    pub bdate: String, // Broadcast Date
    pub edate: String, // End of Date
    pub url: String,
    pub image: String,
    pub badge: String,
}

#[derive(Deserialize)]
struct RawProgram {
    title: String,
    sub_title: String,
    badge: String,
    sub_infos: Vec<String>,
    url: String,
    image: String,
}

/// 【子】1話ごとのデータ（最小単位）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub episode_num: u32,          // パースした半角数字（1, 3, 999など）
    pub is_last: bool,             // 最終回フラグ（true なら色を変えるトリガーにする）
    pub is_badge: bool,
    pub raw_subtitle: String,      // 元の「#3 始まりの事件」といった生の文字列
    pub url: String,               // 個別視聴URL
    pub image: String,
    pub broadcast_date: NaiveDate, // 「西暦のみなら1月1日」ハックを適用した日付
    pub end_label: String,         // 「あと2日」「2週以上」などの配信期限ラベル
}

/// 【親】 ジャンルとタイトルで一意に決まる「番組」の単位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramGroup {
    pub genre: String,          // ドラマ、バラエティ、アニメなどのカテゴリ
    pub title: String,          // 番組タイトル（集約のキー）
    pub station_code: String,   // ローカルマスタから引いた「CX」「TBS」などの局コード
    pub episodes: Vec<Episode>, // この番組に紐づく、配信中の全エピソードのリスト
    // true: 過去作の一挙配信など / false: 今期のリアルタイム現行モノ
    pub is_archive: bool,
}

pub mod cache;
pub mod constants;
pub mod display;
pub mod error;
pub mod fetch;
pub mod launch;
pub mod sift;
pub mod reconfig;
pub mod utils;
