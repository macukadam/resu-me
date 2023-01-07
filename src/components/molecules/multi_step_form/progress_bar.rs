use yew::prelude::*;

#[function_component(ProgressBar)]
pub fn progress_bar() -> Html {
    html! {
        <div class="progress">
          <div aria-valuemax="100" aria-valuemin="0" aria-valuenow="50"
            class="progress-bar progress-bar-striped progress-bar-animated bg-danger" role="progressbar"
            style="width: 0%"></div>
        </div>
    }
}
