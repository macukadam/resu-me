use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

const STYLE: &str = include_str!("css/resume3.css");

#[styled_component(Resume3)]
pub fn resume3() -> Html {
    let stylesheet = Style::new(STYLE).unwrap();
    let (state, _) = use_store::<GlobalState>();
    let img = format!("data:image/png;base64,{}", state.image_data);

    let skills = state.skills.iter().map(|skill| {
        let proficiency = format!("{}%", skill.proficiency);
        html!(
            <div>
                <p>{&skill.skill}</p>
                <div class="progress mb-3">
                    <div class="progress-bar" role="progressbar" style="width: 75%" aria-valuenow={proficiency} aria-valuemin="0" aria-valuemax="100"></div>
                </div>
            </div>
        )
    });

    let education = state.education.iter().map(|education| html!(
        <div class="work-container">
            <h3>{&education.position}</h3>
            <h4><i class="far fa-calendar-alt"></i>{"Jan 2017 to "}<span class="badge badge-info">{"Current"}</span></h4>
            <p>{&education.explanation}</p>
        </div>
    ));

    let experience = state.work_experience.iter().map(|experience| {
        html!(
            <div class="work-container">
                <h3>{&experience.position}</h3>
                <h4><i class="far fa-calendar-alt"></i>{"Jan 2017 to "}<span class="badge badge-info">{"Current"}</span></h4>
                <p>{&experience.explanation}</p>
            </div>
        )
    });

    html! {
        <div id="resume3" class={stylesheet}>
            <div class="container">
                <div class="row">
                    <div class="col-lg-3">
                        <div class="card left-profile-card">
                            <div class="card-body">
                                <div class="text-center">
                                    <img class="img-fluid" alt="Profile Photo" src={img}/>
                                    <h3>{&state.username}</h3>
                                    <p>{&state.job_title}</p>
                                    <div class="d-flex align-items-center justify-content-center mb-3">
                                        <i class="fas fa-star text-info"></i>
                                        <i class="fas fa-star text-info"></i>
                                        <i class="fas fa-star text-info"></i>
                                        <i class="fas fa-star text-info"></i>
                                        <i class="fas fa-star text-info"></i>
                                    </div>
                                </div>
                                <div class="personal-info">
                                    <h3>{"Personal Information"}</h3>
                                    <ul class="personal-list">
                                        <li><i class="fas fa-briefcase"></i><span>{&state.gender.to_string()}</span></li>
                                        <li><i class="fas fa-map-marker-alt "></i><span>{&state.address}</span></li>
                                        <li><i class="far fa-envelope "></i><span>{&state.email}</span></li>
                                        <li><i class="fas fa-mobile "></i><span>{&state.phone}</span></li>
                                    </ul>
                                </div>
                                <div class="skill">
                                    <h3>{"Skills"}</h3>
                                    { for skills }
                                </div>
                                // <div class="languages">
                                //     { for *skills }
                                // </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-lg-9">
                        <div class="card right-profile-card">
                            <div class="card-body">
                                <div class="tab-content" id="pills-tabContent">
                                    <div class="tab-pane fade show active" id="pills-home" role="tabpanel" aria-labelledby="pills-home-tab">
                                        {for experience}
                                    </div>
                                    <div class="tab-pane fade" id="pills-education" role="tabpanel">
                                        { for education }
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
