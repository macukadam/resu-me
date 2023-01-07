use yew::prelude::*;

#[function_component(Preloader)]
pub fn preloader() -> Html {
    html! {
      <div id="preloader-wrapper">
        <div id="preloader"></div>
        <div class="preloader-section section-left"></div>
        <div class="preloader-section section-right"></div>
      </div>
    }
}
