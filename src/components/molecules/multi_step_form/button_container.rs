use yew::prelude::*;

#[function_component(ButtonContainer)]
pub fn button_container() -> Html {
    html! {
        <div id="q-box__buttons">
          <button id="prev-btn" type="button">{"Previous"}</button>
          <button id="next-btn" type="button">{"Next"}</button>
        </div>
    }
}
