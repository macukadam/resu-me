use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

#[function_component(Skill)]
pub fn skill() -> Html {
    let (state, _) = use_store::<GlobalState>();

    let skills = state.skills.iter().map(|skill| {
        let style = format!("width: {}%", &skill.proficiency);
        let aria = format!("{}", &skill.proficiency);
        html! {
            <div class="progress mb-4" style="height:25px;">
              <div class="progress-bar bg-primary text-left pl-2" role="progressbar" style={style} aria-valuenow={aria}
                aria-valuemin="0" aria-valuemax="100">{&skill.skill}</div>
            </div>
        }
    });

    html! {
      <div class="col-md mb-5">
        <h2 class="mb-5">{"Skills"}</h2>
        { for skills }
      </div>
    }
}
