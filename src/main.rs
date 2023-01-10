use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use resume::components::molecules::side_bar::SideBar;
use resume::components::organisms::multi_step_form::MultiStepForm;
use resume::components::organisms::resume::Resume;
use resume::components::organisms::resume2::Resume2;

use resume::bindings::htmltopdf;


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

    let htmltopdf =  Callback::from(|_: MouseEvent| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let element = document.get_element_by_id("resume")
            .unwrap()
            .dyn_into::<HtmlElement>().unwrap();

        htmltopdf(&element);
    });


    html! {
        <div>
            <SideBar child={html!{ <MultiStepForm/> }}/>
            <div id="main">
              <button class="openbtn" onclick={opennav}>{"☰ Open Sidebar"}</button>
              <button class="openbtn" onclick={htmltopdf}>{"☰ Save as PDF"}</button>
              <Resume2/>
            </div>
        </div>
    }
}

fn main() {
    // yew::Renderer::<Test>::new().render();
    yew::Renderer::<TestForm>::new().render();
}
