use yew::prelude::*;

pub fn set_progress_bar(val: u32) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document
        .get_elements_by_class_name("progress-bar")
        .item(0)
        .unwrap()
        .set_attribute("width", &format!("{}%", val))
        .unwrap()
}

#[function_component(ButtonContainer)]
pub fn button_container() -> Html {
    let step_val = use_state(|| 0);

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let steps = document.get_elements_by_class_name("step");
    let cloned_step_val = step_val.clone();
    let cloned_steps = steps.clone();

    let forward: Callback<MouseEvent> = Callback::from(move |_| {
        cloned_steps
            .item(*cloned_step_val)
            .unwrap()
            .set_class_name("step d-none");
        cloned_steps
            .item(*cloned_step_val + 1)
            .unwrap()
            .set_class_name("step d-block");

        cloned_step_val.set(*cloned_step_val + 1);

        set_progress_bar((100 / cloned_steps.length()) * *cloned_step_val);
    });

    let cloned_step_val = step_val.clone();
    let cloned_steps = steps.clone();
    let backward: Callback<MouseEvent> = Callback::from(move |_| {
        cloned_steps
            .item(*cloned_step_val)
            .unwrap()
            .set_class_name("step d-none");
        cloned_steps
            .item(*cloned_step_val - 1)
            .unwrap()
            .set_class_name("step d-block");

        cloned_step_val.set(*cloned_step_val - 1);
        set_progress_bar((100 / cloned_steps.length()) * *cloned_step_val);
    });

    html! {
        <div id="q-box__buttons">
            if *step_val > 0 {
                <button id="prev-btn" onclick={backward} type="button">{"Previous"}</button>
            }

            if *step_val < 4 {
                <button id="next-btn" onclick={forward} type="button">{"Next"}</button>
            }

        </div>
    }
}
