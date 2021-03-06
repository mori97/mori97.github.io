//! Home ページ
use yew::prelude::*;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
              <h2>{"About"}</h2>
              {"はじめまして。Du Yichengと申します。京都大学情報学研究科の"}
              <a href="http://sap.ist.i.kyoto-u.ac.jp/">{"音声メディア研究室(河原研究室)"}</a>
              {"に在籍する大学院生です。"}<br/>
              {"学部時代からずっと機械学習の開発＆研究に取り組んでおり、"}
              {"研究室で音声を対象とする機械学習＆信号処理技術の勉強と研究をしながら、"}
              {"アルバイトで機械学習エンジニアの仕事をしています。"}
              {"ソフトウェアエンジニアの経験もありましたが、ここ2、3年くらいはずっと機械学習エンジニアとして仕事をしており、"}
              {"日立製作所の基礎研究センタを経て、現在は"}
              <a href="https://biome.co.jp/">{"株式会社バイオーム"}</a>
              {"でコンピュータビジョンの研究開発を行っています。"}<br/>
              <a href="https://biome.co.jp/app-biome/">{"いきものコレクションアプリ『バイオーム』"}</a>
              {"の画像認識機能を作っています！もしよかったらダウンロードしてみてください！"}
              <h2>{"取り組み分野"}</h2>
              <ul>
                <li>
                  {"音響信号処理"}
                  <ul>
                    <li>{"音源分離／音声強調"}</li>
                  </ul>
                </li>
                <li>
                  {"コンピュータビジョン"}
                  <ul>
                    <li>{"詳細画像識別"}</li>
                    <li>{"不均衡データ"}</li>
                  </ul>
                </li>
                <li>
                  {"グラフコンボリューション"}
                </li>
              </ul>
              {"最近はHololens2でAR(拡張現実)/MR(複合現実)の勉強もしています！"}
              <h2>{"趣味（プログラミング以外笑）"}</h2>
              <ul>
                <li>{"ロードバイク"}</li>
                <li>
                  {"アニメ"}<br/>
                  {"「らき☆すた」とか、「銀河英雄伝説」（旧作のほう）とか"}
                </li>
                <li>
                  {"料理（下手の横好きレベル）"}<br/>
                  {"簡単なイタリアンが作れるくらい"}
                </li>
              </ul>
            </>
        }
    }
}
