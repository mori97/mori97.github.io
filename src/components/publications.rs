//! Publications ページ
use yew::prelude::*;

pub struct Publications {}

impl Component for Publications {
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
              <h2>{"国際学会"}</h2>
              <ul>
                <li>
                  {"Multi-layered Representation Learning with Edge Restructured Pooling"}<br/>
                  {"Atsushi Miyamoto, Koji Fukuda, and "}
                  <strong>{"Yicheng Du"}</strong><br/>
                  {"ECML-PKDD 2019 Workshop (Graph Embedding and Mining) "}
                  <a href="https://github.com/gem-ecmlpkdd/gem-ecmlpkdd.github.io/raw/ae2663929a12dad8aa7982a598ddf8d0adc3a05e/papers/GEM2019_paper_14.pdf">{"[PDF]"}</a>
                </li>
                <li>
                  {"Semi-supervised Multichannel Speech Separation Based on a Phone- and Speaker-Aware Deep Generative Model of Speech Spectrograms"}<br/>
                  <strong>{"Yicheng Du"}</strong>
                  {", Kouhei Sekiguchi, Yoshiaki Bando, Aditya Arie Nugraha, Mathieu Fontaine, Kazuyoshi Yoshii, and Tatsuya Kawahara"}<br/>
                  {"EUSIPCO 2020 "}
                  <a href="https://ieeexplore.ieee.org/document/9287464">{"[IEEE Xplore]"}</a>
                  <a href="https://www.eurasip.org/Proceedings/Eusipco/Eusipco2020/pdfs/0000870.pdf">{"[PDF]"}</a>
                </li>
                <li>
                  {"Computationally-Efficient Overdetermined Blind Source Separation Based on Iterative Source Steering"}<br/>
                  <strong>{"Yicheng Du"}</strong>
                  {", Robin Scheibler, Masahito Togami, Kazuyoshi Yoshii, and Tatsuya Kawahara"}<br/>
                  {"ICASSP 2021 (Under review)"}
                </li>
              </ul>
              <h2>{"国内学会"}</h2>
              <ul>
                <li>
                  {"話者・音素特徴に基づくマルチチャネル音声分離"}<br/>
                  <strong>{"Yicheng Du，"}</strong>
                  {"關口航平，坂東宜昭，Aditya Arie Nugraha，吉井和佳"}<br/>
                  {"情報処理学会全国大会 (学生奨励賞受賞)"}
                </li>
              </ul>
            </>
        }
    }
}
