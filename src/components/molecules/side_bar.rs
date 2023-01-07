use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

const STYLE: &str = include_str!("css/side_bar.css");

#[derive(Clone, Debug, Default, Properties, PartialEq)]
pub struct Props {
    pub child: Children,
}

#[styled_component(SideBar)]
pub fn side_bar(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    let closenav = Callback::from(|_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        document
            .get_element_by_id("mySidebar")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_attribute("style", "")
            .unwrap();

        document
            .get_element_by_id("main")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_attribute("style", "")
            .unwrap();
    });

    html! {
        <div class={stylesheet}>
            <div id="mySidebar" class="sidebar">
              <a href="javascript:void(0)" class="closebtn" onclick={closenav}>{"x"}</a>
              { props.child.clone() }
            </div>
        </div>
    }
}
