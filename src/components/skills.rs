//! Skills ページ
use yew::prelude::*;

pub struct Skills {}

impl Component for Skills {
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
              <h2>{"プログラミング言語"}</h2>
              {"💪Rust/Python/Julia/C"}<br/>
              {"👌C#/JavaScript(TypeScript)/C++/Java/OCaml/Matlab/Verilog/Visual Basic/Pascal"}
              <h2>{"フレームワーク＆ライブラリ"}</h2>
              {"PyTorch/NumPy/Chainer/Unity/Acitx/React etc."}
              <h2>{"その他"}</h2>
              {"Vim/Docker/LaTeX etc."}
            </>
        }
    }
}
