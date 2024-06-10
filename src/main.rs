use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use resume::components::molecules::side_bar::SideBar;
use resume::components::organisms::multi_step_form::MultiStepForm;
use resume::components::organisms::resume::Resume;
use resume::components::organisms::resume2::Resume2;
// use resume::components::organisms::resume3::Resume3;
// use resume::components::organisms::resume4::Resume4;

use resume::bindings::htmltopdf;

#[function_component(TestForm)]
fn form() -> Html {
    let opennav = Callback::from(|_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        document
            .get_element_by_id("mySidebar")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_attribute("style", "width: 600px;")
            .unwrap();

        document
            .get_element_by_id("main")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_attribute("style", "margin-left: 600px;")
            .unwrap();
    });

    let active_resume_type = use_state(|| 1);
    let active_resume_type_cloned = active_resume_type.clone();
    let active_resume_type_cloned2 = active_resume_type.clone();

    let change_resume_type = Callback::from(move |_| {
        if *active_resume_type == 2 {
            active_resume_type.set(1);
        } else {
            active_resume_type.set(*active_resume_type + 1);
        }
    });

    let htmltopdf = Callback::from(move |_: MouseEvent| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let element = document
            .get_element_by_id(&format!("resume{}", *active_resume_type_cloned))
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        htmltopdf(&element);
    });

    html! {
        <div>
            <SideBar child={html!{ <MultiStepForm/> }}/>
            <div id="main">
                <nav class="navbar navbar-dark bg-dark">
                  <form class="form-inline">
                      <button type="button" class="openbtn btn btn-outline-primary" onclick={opennav}>{"☰ Edit Resume"}</button>
                      <button type="button" class="openbtn btn btn-outline-primary" onclick={htmltopdf}>{"☰ Download as PDF"}</button>
                      <button type="button" class="openbtn btn btn-outline-primary" onclick={change_resume_type}>{"☰ Change type"}</button>
                  </form>
                </nav>
              {
                match *active_resume_type_cloned2 {
                    1 => html!{ <Resume/> },
                    2 => html!{ <Resume2/> },
                    // 3 => html!{ <Resume3/> },
                    // 4 => html!{ <Resume4/> },
                    _ => html!{ <Resume/> },
                }
              }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<TestForm>::new().render();
}
