use yew::prelude::*;
use crate::components::molecules::res1::header::Header;
use crate::components::molecules::res1::nav::Nav;
use crate::components::molecules::res1::work::Work;
use crate::components::molecules::res1::education::Education;
use crate::components::molecules::res1::skill::Skill;
use crate::components::molecules::res1::recent_work::RecentWork;
use crate::components::molecules::res1::footer::Footer;

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        <div id="resume1">
          <Header />
          <Nav />
          <main class="container">
            <div class="row">
                <Work />
                <RecentWork />
            </div>
            <div class="row">
                <Skill />
                <Education />
            </div>
          </main>
          <Footer />
        </div>
    }
}
