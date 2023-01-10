use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

#[function_component(Header)]
pub fn header() -> Html {
    let (state, _) = use_store::<GlobalState>();
    let img = format!("data:image/png;base64,{}", state.image_data);

    html! {
      <header class="bg-primary bg-gradient text-white py-5">
        <div class="container">
          <div class="row">
            <div class="col-md-3 text-left text-md-center mb-3">
              <img class="rounded-circle img-fluid" alt="Profile Photo" src={img}/>
            </div>
            <div class="col-md-9">
              <h1>{&state.username}</h1>
              <h5>{&state.job_title}</h5>
              <p class="border-top pt-3">{&state.job_description} </p>
            </div>
          </div>
        </div>
      </header>
    }
}
