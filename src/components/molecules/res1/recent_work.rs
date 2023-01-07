use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

#[function_component(RecentWork)]
pub fn recent_work() -> Html {
    let (state, _) = use_store::<GlobalState>();

    let images = state.images.iter().map(|img| {
        let image = format!("{}", img);
        html! {
            <div class="col-md mb-3">
                <a href="#">
                  <img class="img-fluid img-thumbnail" src={image}/>
                </a>
            </div>
        }
    });

    html! {
      <div class="col-md">
        <h2 class="mb-5">{"Recent Work"}</h2>
        <div class="row">
            { for images }
        </div>
      </div>
    }
}
