//! Demo ページ
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub demo_id: usize,
}

pub struct Demo {
    props: Props,
}

impl Component for Demo {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.props.demo_id {
             1 => {  // U-Netで音楽パート分離
                 html! {
                     <>
                       <h2>{"U-Netで音楽パート分離"}</h2>
                       <a href="https://github.com/mori97/U-Net_MUSDB18">{"[GitHub]"}</a><br/>
                       {"ディープラーニングで楽曲を４つのパート"}
                       <ul>
                         <li>{"ボーカル(vocal)"}</li>
                         <li>{"ドラム(drums)"}</li>
                         <li>{"ベース(bass)"}</li>
                         <li>{"その他(others)"}</li>
                       </ul>
                       {"に分離します。"}

                       <h3>{"Demo"}</h3>

                       <h4>{"入力"}</h4>
                       <div>
                         <audio controls=true src="audio/unetdemo_mixture.wav">
                           {"Your browser does not support the <code>audio</code> element."}
                         </audio>
                         <figure class="spectrograms">
                           <img src="images/unetdemo_spec_mixture.png" alt="楽曲のスペクトログラム" />
                           <figcaption>{"楽曲のスペクトログラム"}</figcaption>
                         </figure>
                       </div>

                       <h4>{"出力"}</h4>
                       <h5>{"ボーカル(vocal)パート"}</h5>
                       <div>
                         <audio controls=true src="audio/unetdemo_vocal.wav">
                           {"Your browser does not support the <code>audio</code> element."}
                         </audio>
                         <figure class="spectrograms">
                           <img src="images/unetdemo_spec_vocal.png" alt="ボーカル(vocal)のスペクトログラム" />
                           <figcaption>{"ボーカル(vocal)のスペクトログラム"}</figcaption>
                         </figure>
                       </div>

                       <h5>{"ドラム(drums)パート"}</h5>
                       <div>
                         <audio controls=true src="audio/unetdemo_drum.wav">
                           {"Your browser does not support the <code>audio</code> element."}
                         </audio>
                         <figure class="spectrograms">
                           <img src="images/unetdemo_spec_drums.png" alt="ドラム(drums)のスペクトログラム" />
                           <figcaption>{"ドラム(drums)のスペクトログラム"}</figcaption>
                         </figure>
                       </div>

                       <h5>{"ベース(bass)パート"}</h5>
                       <div>
                         <audio controls=true src="audio/unetdemo_bass.wav">
                           {"Your browser does not support the <code>audio</code> element."}
                         </audio>
                         <figure class="spectrograms">
                           <img src="images/unetdemo_spec_bass.png" alt="ベース(Bass)のスペクトログラム" />
                           <figcaption>{"ベース(Bass)のスペクトログラム"}</figcaption>
                         </figure>
                       </div>

                       <h5>{"その他(others)パート"}</h5>
                       <div>
                         <audio controls=true src="audio/unetdemo_others.wav">
                           {"Your browser does not support the <code>audio</code> element."}
                         </audio>
                         <figure class="spectrograms">
                           <img src="images/unetdemo_spec_others.png" alt="その他(others)のスペクトログラム" />
                           <figcaption>{"その他(others)のスペクトログラム"}</figcaption>
                         </figure>
                       </div>
                     </>
                 }
            },
            _ => {
                html! {}
            },
        }
    }
}
