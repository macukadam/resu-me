use std::ops::Deref;

use crate::GlobalState;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub class: String,
    pub value: String,
}

#[function_component(TextInput)]
pub fn app(props: &Props) -> Html {
    let (state, dispatch) = use_store::<GlobalState>();

    let cloned_state = state.clone();
    let onkeyup = Callback::from(move |e: KeyboardEvent| {
        let target = e.target().unwrap();
        let input = target.unchecked_into::<web_sys::HtmlInputElement>();
        let mut state = cloned_state.deref().clone();
        state.username = input.value();
        dispatch.set(state);
    });

    html! {
        <div class={&props.class}>
          <label class="form-label">{&props.value}</label>
          <input {onkeyup} type="text"/>
        </div>
    }
}
