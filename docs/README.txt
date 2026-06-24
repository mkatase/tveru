@note
- Edit Date: 2026-06-25 First Edit
- mdivide -l en -i ./docs/README.txt -o README-en.md
@end
@common
# Tveru

![GitHub release (latest by date)](https://img.shields.io/github/v/release/mkatase/tveru)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/github/license/mkatase/tveru)

@end
#-----------------------------------------------------------------------
@en
## Overview
A lightweight, keyboard-driven TVer drama aggregator and streaming CLI utility.

## Environment
@end
@jp
## 概要
TVerの番組一覧（ドラマ）をコンソールで閲覧するためのCLIユーティリティプログラムです。

## 環境
@end
@common
- cargo 1.96.0 on Fedora 44 (7.0.12-201)
- chromedriver 149.0.7827.155
- mpv 0.41.0
- yt-dlp 2026.06.09

@end
#-----------------------------------------------------------------------
@en
## Features
- Keyboard Driven Interface
- High-density single-line layout grouping all available episodes per program

@end
@jp
## 特徴
- キーボードインタフェース
- 1番組1行に全エピソードを集約する高密度レイアウト

@end
#-----------------------------------------------------------------------
@en
## Build and Install

@end
@jp
## 構築とインストール

@end
@common
```bash
$ cargo install --git https://github.com/mkatase/tveru.git
```

@end
#-----------------------------------------------------------------------
@en
## Execution Mode
| Option        | Description                          | Note |
|:--------------|:-------------------------------------|:-----|
| -d, --day     | Specify target weekday               | *1   |
| -n, --network | Filter by broadcasting network       | *2   |
| -a, --archive | Include archive programs             | *3   |
| -f, --full    | Execute a full crawler sweep         |      |
| -s, --strict  | Hide special/short clips (main only) | *4   |
| -p, --player  | Specify media player / downer script |      |
| -r, --refresh | Force refresh and fetch latest data  |      |
- *1: Allowed strings: are Sun, Mon, Tue, Wed, Thu, Fri, Sat.
- *2: Allowed strings: are NTV, TBS, CX, EX, TX, NHK, OTHER.
- *3: Display non-current/archived pas masterpieces.
- *4: Filters out teasers, pre-shows, and extra clips.

@end
@jp
## 実行モード
| オプション    | 説明                                                     | 備考 |
|:--------------|:---------------------------------------------------------|:-----|
| -d, --day     | 曜日を指定してフィルタリングする                         | *1   |
| -n, --network | 指定した系列局コードでフィルタリングする                 | *2   |
| -a, --archive | 過去作アーカイブを含めて表示する                         | *3   |
| -f, --full    | フルクローリング巡回を実行する                           |      |
| -s, --strict  | 本編のみを非表示（予告や直前SP等を非表示）               | *4   |
| -p, --player  | 使用するプレイヤー/スクリプトを指定する                  |      |
| -r, --refresh | キャッシュを破棄して、最新データを最新データを再取得する |      |
- *1: 指定可能な文字列は、Sun、Mon、Tue、Wed、Thu、Fri、Sat。
- *2: 指定可能な文字列は、NTV、TBS、CX、EX、TX、NHK、OTHER。
- *3: 通常の一覧に加えて、配信中の過去作品アーカイブも表示に含めます。
- *4: 本編以外の「直前SP」「予告」「ナビ」等のクリップを非表示にします。


@end
#-----------------------------------------------------------------------
@en
## TVERU_PLAYER Variable
-The default media player for tveru is mpv. However, you can switch to any player or custom script capable of handling streaming URLs by using the environment variable or the -p option. 
@end
@jp
## TVERU_PLAYER変数
- tveruのデフォルトの動画プレイヤーはmpvですが、ストリーミング対応が
可能なプレイヤー（またはラッパースクリプト）であれば、環境変数、もしくは、
-pオプションを使用することで、簡単に切り替えが可能です。
@end
@common
```bash
 $ mpv https://example.com/xxxx
 $ tveru -n EX 0 3
 $ tveru -n EX -p mpv 0 3
 $ TVERU_PLAYER=mpv tveru -n EX 0 3
```
@end
#-----------------------------------------------------------------------
@en
## Usage

1. [Fetch entries](#usage1)
2. [Force refresh entries](#usage2)
3. [Filter by broadcasting network](#usage3)
4. [Strict mode (main episodes only)](#usage4)
5. [Include archive movies](#usage5)
6. [Filter by weekday](#usage6)
7. [Play/Process target movie](#usage7)
@end
@jp
## 使用方法

1. [エントリを取得する](#usage1)
2. [エントリを再取得する](#usage2)
3. [対象局を指定する](#usage3)
4. [本編のみを表示する](#usage4)
5. [アーカイブを表示する](#usage5)
6. [対象日を指定する](#usage6)
7. [対象動画を見る](#usage7)
@end
#-------------------------------------------------------------------
@en
### <a id="usage1"></a>Fetch entries
@end
@jp
### <a id="usage1"></a>エントリを取得する
@end
@common
```bash
 $ tveru
```
![usage1](./docs/svg/usage1.svg)
@end
#-------------------------------------------------------------------
@en
### <a id="usage2"></a>Force refresh entries
@end
@jp
### <a id="usage2"></a>エントリを再取得する
@end
@common
```bash
 $ tveru --refresh
 or
 $ tveru -r
```
![usage2](./docs/svg/usage2.svg)
@end
#-------------------------------------------------------------------
@en
### <a id="usage3"></a></a>Filter by broadcasting network
@end
@jp
### <a id="usage3"></a>対象局を指定する
@end
@common
```bash
 $ tveru -n TX
 or 
 $ tveru --network TX
```
![usage3](./docs/svg/usage3.svg)
@end
@en
- Multiple networks can be specified using a comma-separated list.
- This option can also be combined with other filtering flags.
@end
@jp
- 対象局は、カンマ区切りを用いて、複数指定が可能です。
- 複数指定は、他オプションとも併用可能です。
@end
@common
```bash
 $ tveru -n TX,EX
 or
 $ tveru --network TX,EX
```
@end
#-------------------------------------------------------------------
@en
### <a id="usage4"></a></a>Strict mode (Main episodes only)
@end
@jp
### <a id="usage4"></a></a>本編のみを表示する
@end
@common
```bash
 $ tveru -n TX -s
 or
 $ tveru --network TX --strict
```
![usage4](./docs/svg/usage4.svg)
@end
#-------------------------------------------------------------------
@en
### <a id="usage5"></a></a>Include archive movies
@end
@jp
### <a id="usage5"></a></a>アーカイブを表示する
@end
@common
```bash
 $ tveru -n TX -a
 or
 $ tveru --network TX --archive
```
![usage4](./docs/svg/usage5.svg)
@end
#-------------------------------------------------------------------
@en
### <a id="usage6"></a></a>Filter by weekday
@end
@jp
### <a id="usage6"></a></a>対象日を指定する
@end
@common
```bash
 $ tveru -n TX -d Fri
 or
 $ tveru --network TX --day Fri
```
@end
#-------------------------------------------------------------------
@en
### <a id="usage7"></a></a>Play/Process target movie
- To play #0 (Final Episode) of "惡の華", pass the Index (leftmost column) and the parsed Episode number.
@end
@jp
### <a id="usage7"></a></a>対象動画を見る
- 「惡の華」の「＃０（最終回）」を見るには、Index（左端）とEpisode（＃項番）を指定します。
@end
@common
```bash
 $ tveru -n TX 1 0
 or
 $ tveru --network TX 1 0
```
@end
@en
- Note: You can pass a download shell script instead of a media player via `-p` to save the video stream locally.
@end
@jp
- 備考: -p オプションにプレイヤーの代わりに自作のダウンロード用シェルスクリプトを指定することで、動画をローカルに自動保存（サルベージ）することも可能です。
@end
#-------------------------------------------------------------------
@en
## Known Issues
- Unordered or Non-contiguous Episode Numbers: TVer's raw metadata embeds episode numbers directly into subtitle text. If parsing detects an unexpected or non-sequential number, it is usually because the text contains numerical characters within the episode title itself (e.g., character names like "Goro(5)", or specific taglines).
- Episode 0 is automatically reserved and mapped as the Final Episode flag [終].
@end
@jp
## 既知の問題
- エピソード番号が０だったり、連番になっていない現象：
 - TVerの生データは、話数とエピソード題名（サブタイトル）が同一文字列内に混在しているため、厳密な切り分けが極めて困難です。そのため、タイトル文字列の中に含まれる野生の数字（固有名詞「五郎(5)」や煽り文句など）を話数として誤認識する場合があります。
- なお、エピソード番号 0 は最終回フラグ [終] として内部的にマッピングされます。
@end
#-------------------------------------------------------------------
@common
## tveru_cache.json
@end
@en
- After execution, the fetched raw data is stored in $XDG_RUNTIME_DIR/tveru_cache.json. You can leverage this file for external pipeline hacks or scripts.
@end
@jp
- Fetch実行後、XDG_RUNTIME_DIR 内に tveru_cache.json という名前で生データをキャッシュします。このJSONファイルを用いて他のシェルスクリプトやパイプラインと連携させることも可能です。
@end
#-------------------------------------------------------------------
@en
## Appendix
- This file is generated by mdivide. Original is [Here](./docs/README.txt).
@end
@jp
## 備考
- 本ファイルは、mdivideにて生成。元ファイルは、[こちら](./docs/README.txt)。
@end
#-------------------------------------------------------------------
@common
## ChangeLog
- ChageLog is [Here](./CHANGELOG.md)
## License
- License is [MIT](./LICENSE)
## 🎧 B.G.M.
@end
@en
- [Borderline(Official)/Madonna](https://www.youtube.com/watch?v=rSaC-YbSDpo)
@end
@jp
- [Borderline(1983)/マドンナ](https://www.youtube.com/watch?v=rSaC-YbSDpo)
@end
