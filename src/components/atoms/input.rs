use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub value: AttrValue,
    pub onkeyup: Callback<KeyboardEvent>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    html! {
        <div class="mt-1">
          <label class="form-label">{&props.label}</label>
          <input value={&props.value} onkeyup={&props.onkeyup} class="form-control"/>
        </div>
    }
}
