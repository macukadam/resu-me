use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub value: AttrValue,
    pub onmouseup: Callback<MouseEvent>,
}

#[function_component(RangeInput)]
pub fn range_input(props: &Props) -> Html {
    html! {
        <div class="mt-2">
          <label class="form-label">{"Proficiency:"}</label>
          <input value={&props.value} onmouseup={&props.onmouseup} type="range" class="form-range" min="0" max="100"/>
        </div>
    }
}
