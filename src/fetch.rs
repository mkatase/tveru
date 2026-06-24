// src/fetch.rs

use thirtyfour::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

use crate::{Element, RawProgram};
use crate::constants;

pub async fn get_scripts_two(driver: &WebDriver, genre: &str, is_full: bool) -> Result<Vec<Element>, WebDriverError> {
    let mut list: Vec<Element> = Vec::new();

    // 巡回するターゲット（クエリパラメータ）のリストを確定
    let targets: Vec<String> = if is_full {
        constants::NETWORKS.iter().map(|net| format!("?tvnetwork={}", net)).collect()
    } else {
        vec![String::new()] // Partialモード（クエリなし無条件）
    };

    // 🎨 Indicatifのマルチバーを関数内部に生成
    let mp = MultiProgress::new();

    // 🚀 親バー
    let total_bar = mp.add(ProgressBar::new(targets.len() as u64));
    total_bar.set_style(ProgressStyle::with_template(
        "🚀 [Total Salvage] {bar:40.green/yellow} {pos}/{len} 局完了"
    ).unwrap());

    // 確定したターゲット分だけループを回す
    for query in targets {
        let url = format!("{}/tags/{}{}", constants::URL_BASE, genre, query);
        driver.goto(&url).await?;
        // 🎯 明示的な「遅延」部分
        // 要素が出現するまで、最大「5秒間」待つ
        match driver
            .query(By::Css(".ContentCard_root__tVcTy"))
            .wait(std::time::Duration::from_secs(5), std::time::Duration::from_millis(200)) // 5秒タイムアウト、200ms間隔で確認
            .first()
            .await 
        {
            Ok(_) => {
            },
            Err(_) => {
                // 5秒経っても画面がタイムアウトなら、次の局へスキップ
                continue;
            }
        }
        // 各局の生JSONを取り出す
        let value = driver.execute(constants::SCRIPT_A, Vec::new()).await?;
        let json_ref = value.json().to_string();
        let elements: Vec<RawProgram> = serde_json::from_str(&json_ref).map_err(WebDriverError::from)?;

        // 局毎の子バーを動的に追加
        let pb = mp.add(ProgressBar::new(elements.len() as u64));
        let template = format!("[{:<7}] [{{bar:30.cyan/blue}}] {{pos}}/{{len}}", genre);
        pb.set_style(ProgressStyle::with_template(&template).unwrap().progress_chars("=> "));

        // 構造体へのマッピング＆格納
        for element in elements {
            let mut info_iter = element.sub_infos.into_iter();
            let bdate = info_iter.next().unwrap_or_default();
            let edate = info_iter.next().unwrap_or_default();

            list.push(Element {
                genre: genre.to_string(),
                title: element.title,
                episode: element.sub_title,
                bdate,
                edate,
                url: element.url,
                image: element.image,
                badge: element.badge,
            });
            pb.inc(1);
        }
        pb.finish_and_clear(); // 終了した局のバーは画面から消去
        total_bar.inc(1);      // 親バーを一歩進める
    }
    total_bar.finish_with_message("All Salvage Done");

    println!("\nスクレイピング完了。合計 {} 件の番組データをマージしました。", list.len());
    Ok(list)
}

pub async fn get_scripts(driver: &WebDriver, genre: &str) -> Result<Vec<Element>, WebDriverError> {

    let mut list: Vec<Element> = Vec::new();


    let template = format!(
        "[{}] [{{bar:40.cyan/blue}}] {{pos}}/{{len}} {{msg}}", genre
    );
    
    let value = driver.execute(constants::SCRIPT_A, Vec::new()).await?;
    let json_ref = value.json().to_string();
    // そのValueを、定義したVec<RawProgram>構造体に変換する
    let elements: Vec<RawProgram> = serde_json::from_str(&json_ref)
        .map_err(WebDriverError::from)?;
    //    .map_err(|e| thirtyfour::prelude::WebDriverError::UnknownError(format!("Failed to parse: {}", e)))?;
    let pb = ProgressBar::new(elements.len() as u64);
    pb.set_style(ProgressStyle::
        with_template(&template)
        .unwrap()
        .progress_chars("=> "));

    for element in elements {
        let mut info_iter = element.sub_infos.into_iter();
        let bdate = info_iter.next().unwrap_or_default();
        let edate = info_iter.next().unwrap_or_default();

        let program = Element {
            genre: genre.to_string(),
            title: element.title,
            episode: element.sub_title,
            bdate,
            edate,
            url: element.url,
            image: element.image,
            badge: element.badge,
        };
        list.push(program);
        pb.inc(1);
    }
    pb.finish_with_message("Done");

    // ループを抜けた後、リストの総数を出す ---
    println!("\nスクレイピング完了。合計 {} 件の番組データを構造体リストに格納しました。", list.len());

    Ok(list)
}
