// tveru/src/cache.rs

use std::fs::File;
use std::path::Path;
use crate::ProgramGroup;

/// XDG_RUNTIME_DIR からキャッシュファイルの絶対パスを取得する
fn get_cache_path() -> String {
    let cache_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/tmp".to_string());
    format!("{}/tveru_cache.json", cache_dir)
}

/// メモリ（tmpfs）上のJSONから構造体を復元する
pub fn load() -> Option<Vec<ProgramGroup>> {
    let path = get_cache_path();
    if Path::new(&path).exists() {
        if let Ok(file) = File::open(&path) {
            if let Ok(data) = serde_json::from_reader::<_, Vec<ProgramGroup>>(file) {
                return Some(data);
            }
        }
    }
    None
}

/// スクレイピング後の構造体をtmpfsに保存する
pub fn save(programs: &Vec<ProgramGroup>) {
    let path = get_cache_path();
    if let Ok(file) = File::create(&path) {
        let _ = serde_json::to_writer_pretty(file, programs);
    }
}
