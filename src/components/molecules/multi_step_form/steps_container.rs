use yew::prelude::*;

use crate::GlobalState;
use yewdux::prelude::*;

use crate::components::atoms::input::Input;
use crate::components::atoms::range_input::RangeInput;
use crate::components::atoms::text_area::TextArea;

use crate::components::molecules::multi_step_form::image_upload::FileDataComponent;

#[function_component(StepsContainer)]
pub fn step_container() -> Html {
    let (state, dispatch) = use_store::<GlobalState>();

    let onkeyup_username = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.username = event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    let onkeyup_jobtitle = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.job_title = event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    let onkeyup_jobdescription =
        dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
            state.job_description = event
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
        });

    let onkeyup_email = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.email = event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    let onkeyup_phone = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.phone = event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    let onkeyup_website = dispatch.reduce_mut_callback_with(|state, event: KeyboardEvent| {
        state.website = event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    });

    let add_work_experience = dispatch.reduce_mut_callback_with(|state, event: MouseEvent| {
        event.prevent_default();
        state.work_experience.push(Default::default());
    });

    let remove_work_experience = dispatch.reduce_mut_callback_with(|state, event: MouseEvent| {
        event.prevent_default();
        state.work_experience.pop();
    });

    let add_education = dispatch.reduce_mut_callback_with(|state, event: MouseEvent| {
        event.prevent_default();
        state.education.push(Default::default());
    });

    let remove_education = dispatch.reduce_mut_callback_with(|state, event: MouseEvent| {
        event.prevent_default();
        state.education.pop();
    });

    let add_skill = dispatch.reduce_mut_callback_with(|state, event: MouseEvent| {
        event.prevent_default();
        state.skills.push(Default::default());
    });

    let remove_skill = dispatch.reduce_mut_callback_with(|state, event: MouseEvent| {
        event.prevent_default();
        state.skills.pop();
    });

    let img = format!("data:image/png;base64,{}", state.image_data.clone());

    html! {
        <div id="steps-container">

          <div class="step d-block">
            <FileDataComponent/>
            <img class="rounded-circle img-fluid" alt="Profile Photo" src={img}/>
            <h4>{"Introduction"}</h4>
            <Input value={state.username.clone()} onkeyup={onkeyup_username} label="Name:" />
            <Input value={state.job_title.clone()} onkeyup={onkeyup_jobtitle} label="Job Title:" />
            <TextArea value={state.job_description.clone()} onkeyup={onkeyup_jobdescription} label="Job Description:" />
          </div>
          <div class="step">
            <h4>{"Contact infortmation:"}</h4>
            <Input value={state.email.clone()} onkeyup={onkeyup_email} label="Email:" />
            <Input value={state.phone.clone()} onkeyup={onkeyup_phone} label="Phone:" />
            <Input value={state.website.clone()} onkeyup={onkeyup_website} label="Website:" />
          </div>
          <div class="step">
            <h4>{"Work history"}</h4>
            { for state.work_experience.iter().enumerate().map(|(index, work)| {

            let onkeyup_workexperience_position =
                dispatch.reduce_mut_callback_with(move |state, event: KeyboardEvent| {
                state.work_experience[index].position = event
                    .target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value();
            });

            let onkeyup_workexperience_explanation =
                dispatch.reduce_mut_callback_with(move |state, event: KeyboardEvent| {
                    state.work_experience[index].explanation = event
                        .target_unchecked_into::<web_sys::HtmlInputElement>()
                        .value();
            });

              html! {
                <div>
                  <br/>
                  <Input value={work.position.clone()} onkeyup={&onkeyup_workexperience_position} label="Position:" />
                  <TextArea value={work.explanation.clone()} onkeyup={&onkeyup_workexperience_explanation} label="Explanation:" />
                </div>
              }
            })}

            <br/>
            <button onclick={add_work_experience} style="font-size:16px"><i class="fa fa-plus"></i></button>
            <button onclick={remove_work_experience} style="font-size:16px"><i class="fa fa-minus"></i></button>
          </div>
          <div class="step">
            <h4>{"Education history"}</h4>

            { for state.education.iter().enumerate().map(|(index, education)| {

            let onkeyup_education_school =
                dispatch.reduce_mut_callback_with(move |state, event: KeyboardEvent| {
                    state.education[index].position = event
                        .target_unchecked_into::<web_sys::HtmlInputElement>()
                        .value();
            });

            let onkeyup_education_explanation =
                dispatch.reduce_mut_callback_with(move |state, event: KeyboardEvent| {
                    state.education[index].explanation = event
                        .target_unchecked_into::<web_sys::HtmlInputElement>()
                        .value();
            });


              html! {
                <div>
                    <br/>
                    <Input value={education.position.clone()} onkeyup={onkeyup_education_school} label="Education name:" />
                    <TextArea value={education.explanation.clone()} onkeyup={onkeyup_education_explanation} label="Explanation:" />
                </div>
              }
            })}


            <br/>
            <button onclick={add_education} style="font-size:16px"><i class="fa fa-plus"></i></button>
            <button onclick={remove_education} style="font-size:16px"><i class="fa fa-minus"></i></button>
          </div>
          <div class="step">
            <h4>{"Skills"}</h4>

            { for state.skills.iter().enumerate().map(|(index, skill)| {

            let onkeyup_skills_skill = dispatch.reduce_mut_callback_with(move |state, event: KeyboardEvent| {
                state.skills[index].skill = event
                    .target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value();
            });

            let onmouseup_skills_proficiency =
                dispatch.reduce_mut_callback_with(move |state, event: MouseEvent| {
                    state.skills[index].proficiency = event
                        .target_unchecked_into::<web_sys::HtmlInputElement>()
                        .value();
            });


              html! {
                <div>
                    <br/>
                    <Input value={skill.skill.clone()} onkeyup={onkeyup_skills_skill} label="Skill:" />
                    <RangeInput value={skill.proficiency.clone()} onmouseup={onmouseup_skills_proficiency} label="Proficiency:" />
                </div>
              }
            })}


            <br/>
            <button onclick={add_skill} style="font-size:16px"><i class="fa fa-plus"></i></button>
            <button onclick={remove_skill} style="font-size:16px"><i class="fa fa-minus"></i></button>
          </div>
        </div>
    }
}
