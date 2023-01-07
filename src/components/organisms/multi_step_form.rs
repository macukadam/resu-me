use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::multi_step_form::progress_bar::ProgressBar;
use crate::components::molecules::multi_step_form::steps_container::StepsContainer;
use crate::components::molecules::multi_step_form::button_container::ButtonContainer;
use crate::components::molecules::multi_step_form::preloader::Preloader;

const STYLE: &str = include_str!("css/style.css");

#[styled_component(MultiStepForm)]
pub fn multi_step_form() -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <div class={stylesheet}>
            <ProgressBar />
            <div id="qbox-container">
              <form class="needs-validation" id="form-wrapper" method="post" name="form-wrapper" novalidate=true>
                <StepsContainer />
                <ButtonContainer />
              </form>
            </div>
            <Preloader />
        </div>
    }
}
