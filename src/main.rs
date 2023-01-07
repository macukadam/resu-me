use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
// use wasm_bindgen::prelude::*;
// use gloo::console::log;
use yewdux::prelude::*;

use resume::GlobalState;

use resume::components::atoms::text_input::TextInput;
use resume::components::molecules::side_bar::SideBar;
use resume::components::organisms::multi_step_form::MultiStepForm;
use resume::components::organisms::resume::Resume;

#[function_component(Test)]
fn test_input() -> Html {
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
        <div class="container">
            <div class="row">
                <div class="col-2">
                    <TextInput {onkeyup}/>
                    <p>{&state.username}</p>
                </div>
                <div class="col-6">
                    <Resume/>
                </div>
            </div>
        </div>
    }
}

#[function_component(TestForm)]
fn form() -> Html {
    let opennav = Callback::from(|_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        document.get_element_by_id("mySidebar")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_attribute("style", "width: 600px;")
            .unwrap();

        document.get_element_by_id("main")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_attribute("style", "margin-left: 600px;")
            .unwrap();

    });

    html! {
        <div>
            <SideBar child={html!{ <MultiStepForm/> }}/>
            <div id="main">
              <button class="openbtn" onclick={opennav}>{"☰ Open Sidebar"}</button>
              <Resume/>
            </div>
        </div>
    }
}

fn main() {
    // yew::Renderer::<Test>::new().render();
    yew::Renderer::<TestForm>::new().render();
}
