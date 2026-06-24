// tveru/src/launch.rs

use crate::{Args, ProgramGroup};
use crate::constants;
use crate::error::Result;

pub fn to_media(y: &[ProgramGroup], args: &Args) -> Result<()> {

    if let (Some(idx), Some(target_ep)) = (args.index, args.episode) {
        if let Some(program) = y.get(idx) {
            
            // 画面表示と同じクローンを作り、ソートと重複排除を行う
            let mut valid_episodes = program.episodes.clone();
            valid_episodes.sort_by_key(|ep| ep.episode_num);
            valid_episodes.dedup_by_key(|ep| ep.episode_num);

            if let Some(episode) = valid_episodes.iter().find(|ep| ep.episode_num == target_ep) {
                let player = std::env::var("TVERU_PLAYER").unwrap_or(args.player.clone());
                
                let url = if episode.url.starts_with('/') {
                    format!("{}{}", constants::URL_BASE, episode.url)
                } else {
                    episode.url.clone()
                };

                println!("🎬 Launching {} for: {} #{}", player, program.title, episode.episode_num);
                let _ = std::process::Command::new(player).arg(&url).spawn();
                
                return Ok(()); 
            } else {
                println!("❌ 番組「{}」の中に、指定された話数 #{} が見つかりません。", program.title, target_ep);
                let available: Vec<String> = program.episodes.iter().map(|ep| format!("#{}", ep.episode_num)).collect();
                println!("   (データ上に存在する話数: {})", available.join(", "));
            }
        } else {
            println!("❌ 指定されたインデックス [{}] は現在のフィルター結果（0〜{}）の範囲外です。", idx, y.len() - 1);
        }
        return Ok(());
    }
    Ok(())
}
