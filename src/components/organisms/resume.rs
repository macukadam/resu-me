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
        <div>
          <Header />
          <Nav />
          <main class="container">
            <div class="row">
                <Work />
                <Education />
            </div>
            <div class="row">
                <Skill />
                <RecentWork />
            </div>
          </main>
          <Footer />
        </div>
    }
}
