use yew::prelude::*;

use crate::GlobalState;
use yewdux::prelude::*;


#[function_component(StepsContainer)]
pub fn step_container() -> Html {
    let (state, dispatch) = use_store::<GlobalState>();

    let onkeyup_username = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.username = event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    let onkeyup_jobtitle = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.job_title= event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    let onkeyup_jobdescription = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.job_description= event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    html! {
        <div id="steps-container">
          <div class="step d-block">
            <h4>{"Provide us with your personal information"}</h4>
            <div class="mt-1">
              <label class="form-label">{"Name:"}</label>
              <input value={state.username.clone()} onkeyup={onkeyup_username} class="form-control"/>
            </div>
            <div class="mt-2">
              <label class="form-label">{"Job Title:"}</label>
              <input value={state.job_title.clone()} onkeyup={onkeyup_jobtitle} class="form-control"/>
            </div>
            <div class="mt-2">
              <label class="form-label">{"Job Description:"}</label>
              <textarea value={state.job_description.clone()} rows="4" cols="50" onkeyup={onkeyup_jobdescription} class="form-control"/>
            </div>
          </div>
        </div>
    }
}
