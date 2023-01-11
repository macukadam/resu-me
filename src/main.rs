use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use resume::components::molecules::side_bar::SideBar;
use resume::components::organisms::multi_step_form::MultiStepForm;
use resume::components::organisms::resume::Resume;
use resume::components::organisms::resume2::Resume2;
use resume::components::organisms::resume3::Resume3;

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

    let active_resume_type = use_state(|| 1);
    let active_resume_type_cloned = active_resume_type.clone();

    let change_resume_type = Callback::from(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        if *active_resume_type == 3 {
            active_resume_type.set(1);
        } else {
            active_resume_type.set(*active_resume_type + 1);
        }

        for i in 1..=3 {
            document.get_element_by_id(&format!("resume{}", i))
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .set_attribute("style", "display: none;")
                .unwrap();
        }

        document.get_element_by_id(&format!("resume{}", *active_resume_type))
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_attribute("style", "display: block;")
            .unwrap();

    });


    let htmltopdf =  Callback::from(move |_: MouseEvent| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let element = document.get_element_by_id(&format!("resume{}", *active_resume_type_cloned))
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
              <button class="openbtn" onclick={change_resume_type}>{"☰ Change resume type"}</button>
              <Resume/>
              <Resume2/>
              <Resume3/>
            </div>
        </div>
    }
}

fn main() {
    // yew::Renderer::<Test>::new().render();
    yew::Renderer::<TestForm>::new().render();
}
