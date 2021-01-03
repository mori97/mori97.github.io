//! アンコンや名前、Email などを記載する Bio サイドブロック
use yew::prelude::*;

pub struct BioSideBlock {}

impl Component for BioSideBlock {
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
              <div class="my-photo">
                <img src="images/my_photo.jpg" alt="my photo" />
              </div>
              <div class="bio">
                <h3>{"Du Yicheng"}</h3>
                <p>{"Master student"}<br/>{"ML engineer"}</p>
                <div>
                  <i class="fas fa-university"></i>
                  <a href="https://www.kyoto-u.ac.jp/ja">{" Kyoto Univ."}</a>
                </div>
                <div>
                  <i class="fas fa-briefcase"></i>
                  <a href="https://biome.co.jp/">{" BIOME Inc."}</a>
                </div>
                <div>
                  <i class="fas fa-location-arrow"></i>
                  {" Kyoto, Japan"}
                </div>
                <div>
                  <i class="far fa-envelope"></i>
                  <a href="mailto:tottexi97131@gmail.com">{" Email"}</a>
                </div>
              </div>
            </>
        }
    }
}
