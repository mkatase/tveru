// tveru/src/utils/episode.rs

use crate::constants::DETOX_DICTIONARY;

/// 文字列全体の全角数字や、パースの邪魔になる「第」「話」などを事前に半角・標準化する
fn clean_subtitle(s: &str) -> String {
    // 小文字化
    let mut cleaned = s.to_lowercase();

    // 辞書をループで回して置換
    for (target, replacement) in DETOX_DICTIONARY.iter() {
        cleaned = cleaned.replace(target, replacement);
    }

    // 全角数字を半角にする
    cleaned.chars()
        .map(|c| match c {
            // 全角数字を半角数字に置換
            '０'..='９' => ((c as u32) - '０' as u32 + '0' as u32) as u8 as char,
            _ => c,
        })
        .collect::<String>()
        .replace("第", "")
        .replace("話", "")
        .replace("回", "")
        .replace("エピソード", "")
}

/// 戻り値を (話数, 最終回かどうか) に変換
pub fn parse_episode_number(subtitle_string: &str) -> (u32, bool) {
    // 文字列を綺麗にする（全角半角の標準化や「第」「話」の除去）
    let cleaned = clean_subtitle(subtitle_string);

    // 最終回フラグをチェック
    let mut is_last = cleaned.contains("最終");

    // 「直前SP」は本編の最終回じゃないので、フラグをfalseへ
    if cleaned.contains("直前") || cleaned.contains("予告") {
        is_last = false;
    }
    // 「30分」や「2026年」などの無関係な数字の塊を消去
    let re_noise = regex::Regex::new(r"\d+[分秒年]").unwrap();
    let safer_cleaned = re_noise.replace_all(&cleaned, "");

    let re_fallback = regex::Regex::new(r"(\d+)").unwrap();
    
    if let Some(caps) = re_fallback.captures(&safer_cleaned) {
        if let Some(m) = caps.get(1) {
            if let Ok(num) = m.as_str().parse::<u32>() {
                return (num, is_last); // 話数が見つかったら、最終回フラグと一緒に返す
            }
        }
    }

    // 数字がないSPや特別編で、かつ「最終回」と書かれているケースも含めてフォールバック
    (0, is_last)
}
