
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub value: AttrValue,
    pub onkeyup: Callback<KeyboardEvent>,
}

#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    html! {
        <div class="mt-1">
            <label class="form-label">{&props.label}</label>
            <textarea value={&props.value} rows="4" cols="50" onkeyup={&props.onkeyup} class="form-control"/>
        </div>
    }
}
