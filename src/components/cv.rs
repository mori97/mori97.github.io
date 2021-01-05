//! CV ページ
use yew::prelude::*;

pub struct CV {}

/// CollapsibleContent と同じスタイルの詳細情報がない項目を生成する
fn simple_item(title: &str) -> Html {
    html! {
        <div class="collapsible_title">
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
        html! {
            <>
              <h2>{"学歴"}</h2>
              <div class="content_cv">
                {simple_item("2016年4月〜2020年3月：京都大学 工学部 情報学科 計算機科学コース")}
                {simple_item("2020年4月〜：京都大学大学院 情報学研究科 知能情報学専攻")}
              </div>
              <h2>{"職歴"}</h2>
              <div class="content_cv">
                {simple_item("2017年3月〜2018年10月：株式会社シー・オー・コンヴ")}
                {simple_item("2018年7月〜2019年4月：株式会社日立製作所 研究開発グループ 基礎研究センタ 京大ラボ")}
                {simple_item("2018年8月〜現在：株式会社バイオーム")}
                {simple_item("2019年10月〜現在：（OA，TA）京都大学")}
                {simple_item("2020年8月〜2020年9月：（インターンシップ）LINE株式会社")}
              </div>
            </>
        }
    }
}
