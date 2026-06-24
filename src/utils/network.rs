// src/utils/network.rs

/// 系列局の振り分け
pub fn normalize_station_code(bdate_str: &str) -> &'static str {
    // フジテレビ系列 (CX)
    if bdate_str.contains("フジテレビ") 
        || bdate_str.contains("カンテレ") 
        || bdate_str.contains("東海テレビ") 
        || bdate_str.contains("サガテレビ") 
        || bdate_str.contains("テレビ西日本") 
        || bdate_str.contains("BSフジ") 
    {
        "CX"
    } 
    // TBS系列 (TBS)
    else if bdate_str.contains("TBS") 
        || bdate_str.contains("BS-TBS")
        || bdate_str.contains("毎日放送") 
        || bdate_str.contains("MBS") 
        || bdate_str.contains("CBC") 
        || bdate_str.contains("IBC") 
    {
        "TBS"
    } 
    // 日本テレビ系列 (NTV)
    else if bdate_str.contains("日本テレビ") 
        || bdate_str.contains("日テレ") 
        || bdate_str.contains("読売テレビ") 
        || bdate_str.contains("ytv")
        || bdate_str.contains("中京テレビ") 
        || bdate_str.contains("西日本放送") 
        || bdate_str.contains("福島中央テレビ") 
    {
        "NTV"
    } 
    // テレビ朝日系列 (EX)
    else if bdate_str.contains("テレビ朝日") 
        || bdate_str.contains("テレ朝") 
        || bdate_str.contains("ABC")
        || bdate_str.contains("ABCテレビ")
        || bdate_str.contains("メ〜テレ")
        || bdate_str.contains("BS朝日")
    {
        "EX"
    } 
    // テレビ東京系列 (TX)
    else if bdate_str.contains("テレビ東京") 
        || bdate_str.contains("テレ東") 
        || bdate_str.contains("テレビ大阪")
        || bdate_str.contains("テレビ愛知")
        || bdate_str.contains("テレビ北海道")
        || bdate_str.contains("テレビせとうち")
        || bdate_str.contains("TVQ")
    {
        "TX"
    } 
    // NHK
    else if bdate_str.contains("NHK") {
        "NHK"
    } 
    // その他（BS11、BS12、WOWOW、独立U局など、系列に属さないもの）
    else {
        "OTHER" 
    }
}
