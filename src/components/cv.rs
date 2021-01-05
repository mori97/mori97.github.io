//! CV ページ
use crate::components::CollapsibleContent;
use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct CV {}

/// CollapsibleContent と同じような詳細情報がない項目を生成する
fn simple_item(title: &str) -> Html {
    html! {
        <div>
          <i class="fas fa-square"></i>{" "}{title}
        </div>
    }
}

impl Component for CV {
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
        let coconv_content = html! {
            <div>
              {"主にPythonとC言語でソフトウェア開発に従事しました。"}
            </div>
        };
        type Anchor = RouterAnchor<AppRoute>;
        let hitachi_content = html! {
            <div>
              {"機械学習エンジニアとしてPyTorchによるディープラーニング（深層学習）の論文の再現実装やライブラリの整備、"}
              {"研究補助の業務に従事しました。"}<br/>
              {"日立製作所にいた頃は主にグラフ畳み込みニューラルネットワーク（GCN）の研究に関わっており、"}
              {"共著で論文を国際学会にも投稿しました。"}
              {"（Miyamoto2019, see "}<Anchor route=AppRoute::Publications>{"Publications"}</Anchor>{"）。"}
            </div>
        };
        let biome_content = html! {
            <div>
              {"株式会社バイオームでは機械学習エンジニアとして"}
              <a href="https://biome.co.jp/app-biome/">{"いきものコレクションアプリ『バイオーム』"}</a>
              {"に搭載された生物の画像認識システムの開発に従事しました。"}<br/>
              {"入社した当時には、設備も学習データもノーハウもないという状態でしたが、"}
              {"要件定義、GPUサーバの購入、実装など全部自力で一から行い、様々な課題を解決して、"}
              {"なんとかこの生物画像認識機能の性能を実用レベルに上げ、実際のアプリに載せることができました。"}<br/>
              {"自分が持っている機械学習の技術で、一から設計・実装、デプロイ・製品化まで行うという貴重な経験ができ、"}
              {"とても有意義な仕事だと思っています。"}
              <figure class="biome_imgrec">
                <img src="images/biome_imgrec.png" alt="「バイオーム」の生物画像認識機能" />
                <figcaption>{"「バイオーム」の生物画像認識機能"}</figcaption>
              </figure>
            </div>
        };
        let line_content = html! {
            <div>
              {"データ分析・機械学習の研究開発を行う専門組織であるData LabsのSpeechチームにて"}
              {"音声認識のアプリケーションで使われる音源分離技術の研究開発を行いました。"}<br/>
              {"インターンシップの成果はICASSPという国際学会に投稿され、現在査読中です。"}
            </div>
        };
        html! {
            <>
              <h2>{"学歴"}</h2>
              <div class="content_cv">
                {simple_item("2016年4月〜2020年3月：京都大学 工学部 情報学科 計算機科学コース")}
                {simple_item("2020年4月〜：京都大学大学院 情報学研究科 知能情報学専攻")}
              </div>
              <h2>{"職歴"}</h2>
              <div class="content_cv">
                <CollapsibleContent
                    title="2017年3月〜2018年10月：株式会社シー・オー・コンヴ"
                    content=coconv_content />
                <CollapsibleContent
                    title="2018年7月〜2019年4月：株式会社日立製作所 研究開発グループ 基礎研究センタ 京大ラボ"
                    content=hitachi_content />
                <CollapsibleContent
                    title="2018年8月〜現在：株式会社バイオーム"
                    content=biome_content />
                {simple_item("2019年10月〜現在：（OA，TA）京都大学")}
                <CollapsibleContent
                    title="2020年8月〜2020年9月：（インターンシップ）LINE株式会社"
                    content=line_content />
              </div>
            </>
        }
    }
}
