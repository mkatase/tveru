// src/reconfig.rs

use std::collections::{HashMap, HashSet};
use chrono::Datelike;

use crate::{Element, ProgramGroup};
use crate::Episode;
use crate::utils::{date_parser, ep_parser, network};

/// フラットなベクタから、番組ごとの親子構造に組み替える関数
pub fn aggregate_programs(raw_list: Vec<Element>) -> Vec<ProgramGroup> {
    // キーは (ジャンル, タイトル) のペアにして重複をまとめる
    let _raw_count = raw_list.len();
    let mut grouped: HashMap<(String, String), ProgramGroup> = HashMap::new();

    //  OTHERに判定された生局名をストック
    let mut unknown_stations = HashSet::new();
    //  実行時の現在の西暦を動的に取得
    let current_year = chrono::Local::now().year();

    for raw in raw_list {
        // エピソード番号と最終回フラグをパース
        let (ep_num, is_last) = ep_parser::parse_episode_number(&raw.episode);
        // 日付を型に
        let b_date = date_parser::parse_broadcast_date(&raw.bdate);
        // 系列コードに変換
        let station_group = network::normalize_station_code(&raw.bdate);

        // OTHER だったら、名前を捕獲
        if station_group == "OTHER" {
            unknown_stations.insert(raw.bdate.clone());
        }

        // 実行時の西暦より前の年、つまり過去作（2021年など）ならアーカイブ
        let is_this_episode_archive = b_date.year() < current_year;

        let is_badge = !raw.badge.is_empty();
        // 仮データで Episodeを作成
        let episode = Episode {
            episode_num: ep_num,
            is_last,
            is_badge,
            raw_subtitle: raw.episode.clone(),
            url: raw.url.clone(),
            image: raw.image.clone(),
            broadcast_date: b_date,
            end_label: raw.edate.clone(),
        };

        // HashMapへの流し込み（あればpush、なければ新規作成）
        let key = (station_group.to_string(), raw.title.clone());
        
        let program = grouped.entry(key)
            .or_insert_with(|| ProgramGroup {
                genre: raw.genre.clone(),
                title: raw.title.clone(),
                station_code: station_group.to_string(),
                episodes: Vec::new(),
                is_archive: false,
            });
        // 1話でも過去作のフラグが立っていたら、グループ全体をアーカイブ
        if is_this_episode_archive {
            program.is_archive = true;
        }
    
        program.episodes.push(episode);
    }

    // HashMapの値[ProgramGroup]だけをVecにして返す
    let rolled_up: Vec<ProgramGroup> = grouped.into_values().collect();

    // デバッグ出力を専用モジュール
    // report::dump_aggregation_report(raw_count, &rolled_up, &unknown_stations);

    rolled_up
}
