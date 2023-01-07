use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

#[function_component(Education)]
pub fn education() -> Html {
    let (state, _) = use_store::<GlobalState>();

    let educations = state.education.iter().map(|education| {
        html! {
          <li>
            <h6 class="text-primary">{&education.position}</h6>
            <p>{&education.explanation}</p>
          </li>
        }
    });

    html! {
          <div class="col-md mb-5">
            <h2 class="mb-5">{"Education"}</h2>
            <ul>
                { for educations }
            </ul>
          </div>
    }
}
