use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub elem: Children,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    html! {
        <div>
            <h1 onclick={onclick}>{ "Hello World!" }</h1>
            { props.elem.clone() }
        </div>
    }
}
