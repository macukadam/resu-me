use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

#[function_component(Work)]
pub fn work() -> Html {
    let (state, _) = use_store::<GlobalState>();

    let work_experiences = state.work_experience.iter().map(|work| {
        html! {
          <li>
            <h6 class="text-primary">{&work.position}</h6>
            <p>{&work.explanation}</p>
          </li>
        }
    });

    html! {
      <div class="col-md mb-5">
        <h2 class="mb-5">{"work experience"}</h2>
        <ul>
            {for work_experiences}
        </ul>
      </div>
    }
}
