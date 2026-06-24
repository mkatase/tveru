// src/constants.rs

pub const NETWORKS: [&'static str; 6] =
    ["nns", "exnetwork", "jnn", "txn", "fns", "independence"];

pub const URL_BASE: &str = "https://tver.jp";

/// 非本編であることを明示するための色分け（マゼンダピンク）
pub const NO_DISP: &str = "#ff00cc";

/// TVer表記を標準化するための辞書
pub const DETOX_DICTIONARY: [(&str, &str); 24] = [
    // 英語・その他表記の翻訳
    ("final episode", "最終回"),
    ("last ep", "最終回"),
    ("最終飯", "最終回"),
    // 漢数字「十」を跨ぐパターンの標準化
    ("三十", "3"), ("二十", "2"), 
    ("十一", "11"), ("十二", "12"), ("十三", "13"), ("十四", "14"),
    ("十五", "15"), ("十六", "16"), ("十七", "17"), ("十八", "18"), ("十九", "19"),
    ("一", "1"), ("二", "2"), ("三", "3"), ("四", "4"), 
    ("五", "5"), ("六", "6"), ("七", "7"), ("八", "8"), ("九", "9"),
    ("十", "10"),
];

pub const SCRIPT_A: &str =
    r"return Array.from(document.querySelectorAll('.ContentCard_root__tVcTy')).map(el => {
        const title    = el.querySelector('.Title_root__4Nr7I')?.innerText || '';
        const subTitle = el.querySelector('.SubTitle_root__K_Y7n')?.innerText || '';
        const badge    = el.querySelector('.Badge_label__BTVQW')?.innerText || '';
        const subInfos = Array.from(el.querySelectorAll('.SubInfo_root__KjyCp')).map(s => s.innerText);
        const url      = el.getAttribute('href') || '';
        const image    = el.querySelector('.Thumbnail_img__DBLSJ').getAttribute('src') || '';
        return { title, sub_title: subTitle, badge, sub_infos: subInfos, url, image };
   });";
