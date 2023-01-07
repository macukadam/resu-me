use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onkeyup: Callback<KeyboardEvent>,
}

#[function_component(TextInput)]
pub fn app(props: &Props) -> Html {

    html! {
        <input onkeyup={&props.onkeyup} type="text" />
    }
}
