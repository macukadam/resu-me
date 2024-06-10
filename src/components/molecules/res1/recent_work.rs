use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

#[function_component(RecentWork)]
pub fn recent_work() -> Html {
    let (state, _) = use_store::<GlobalState>();

    let recent_work = state.recent_work.iter().map(|recent| {
        html! {
            <div>
                <h6 class="text-primary">
                    <a href={recent.link.clone()}>{&recent.project}</a>
                </h6>
                <p style="white-space: pre-wrap;font-size:11pt">{&recent.explanation}</p>
            </div>
        }
    });

    html! {
      <div class="col-md">
        <h2 class="mb-5">{"Recent Work"}</h2>
        <div class="row">
            { for recent_work }
        </div>
      </div>
    }
}
