use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

const STYLE: &str = include_str!("css/resume2.css");

#[styled_component(Resume2)]
pub fn resume2() -> Html {
    let stylesheet = Style::new(STYLE).unwrap();
    let (state, _) = use_store::<GlobalState>();
    let img = format!("data:image/png;base64,{}", state.image_data);

    let skills = state.skills.iter().map(|skill| {
        html!(
            <div class="progress bg-dark">
                <div class="progress-bar bg-white" role="progressbar"
                     style={format!("width:{}%", skill.proficiency)}
                     aria-valuenow={skill.proficiency.clone()}
                     aria-valuemin="0"
                     aria-valuemax="100">
                  <div class="progress-bar-title text-white">{&skill.skill}</div>
                  <span class="progress-bar-number text-white">{&skill.proficiency}{"%"}</span>
                </div>
            </div>
        )
    });

    let education = state.education.iter().map(|education| {
        html!(
            <div class="jobster-timeline-item">
              <div class="jobster-timeline-cricle">
                <i class="far fa-circle"></i>
              </div>
              <div class="jobster-timeline-info">
                <div class="dashboard-timeline-info">
                  <h6 class="mb-2">{&education.position}</h6>
                  <p style="white-space: pre-wrap;">{&education.explanation}</p>
                </div>
              </div>
            </div>
        )
    });

    let experience = state.work_experience.iter().map(|experience| {
        html!(
        <div class="jobster-timeline-item">
          <div class="jobster-timeline-cricle">
            <i class="far fa-circle"></i>
          </div>
          <div class="jobster-timeline-info">
            <div class="dashboard-timeline-info">
              <h6 class="mb-2">{&experience.position}</h6>
              <p style="white-space: pre-wrap;">{&experience.explanation}</p>
            </div>
          </div>
        </div>
        )
    });

    let recent_work = state.recent_work.iter().map(|recent| {
        html!(
            <div class="jobster-timeline-item mb-0">
              <div class="jobster-timeline-cricle">
                <i class="far fa-circle"></i>
              </div>
              <div class="jobster-timeline-info">
                <div class="dashboard-timeline-info">
                  <h6 class="mb-2">
                    <a href={recent.link.clone()}>{&recent.project}</a>
                  </h6>
                  <span>{&recent.explanation}</span>
                </div>
              </div>
            </div>
        )
    });

    html! {
        <div id="resume2" class={stylesheet}>
            <div class="container">
                <div class="row justify-content-center">
                    <div class="col-md-9">
                      <div class="row align-items-center">
                        <div class="col-lg-5">
                          <div class="resume-base bg-primary user-dashboard-info-box p-4">
                            <div class="profile">
                              <div class="jobster-user-info">
                                <div class="profile-avatar">
                                  <img class="img-fluid" alt="Profile Photo" src={img}/>
                                </div>
                                <div class="profile-avatar-info mt-3">
                                  <h5 class="text-white">{&state.username}</h5>
                                </div>
                              </div>
                            </div>
                            <div class="about-candidate border-top">
                              <div class="candidate-info">
                                <h6 class="text-white">{"Email:"}</h6>
                                <p class="text-white">{&state.email}</p>
                              </div>
                              <div class="candidate-info">
                                <h6 class="text-white">{"Phone:"}</h6>
                                <p class="text-white">{&state.phone}</p>
                              </div>
                              <div class="candidate-info">
                                <h6 class="text-white">{"Date Of Birth:"}</h6>
                                <p class="text-white">{&state.dob}</p>
                              </div>
                              <div class="candidate-info">
                                <h6 class="text-white">{"Address:"}</h6>
                                <p class="text-white">{&state.address}</p>
                              </div>
                              <div class="candidate-info">
                                <h6 class="text-white">{"Gender:"}</h6>
                                <p class="text-white">{&state.gender.to_string()}</p>
                              </div>
                            </div>
                            <div class="mt-0">
                              <h5 class="text-white">{"Professional Skill:"}</h5>
                              { for skills }
                            </div>
                          </div>
                        </div>
                        <div class="col-lg-7">
                          <div class="resume-experience">
                            <div class="timeline-box">
                              <h5 class="resume-experience-title">{"Education:"}</h5>
                              <div class="jobster-candidate-timeline">
                                { for education }
                              </div>
                            </div>
                            <div class="timeline-box mt-4">
                              <h5 class="resume-experience-title">{"Work experience:"}</h5>
                              <div class="jobster-candidate-timeline">
                                { for experience }
                              </div>
                            </div>
                            <div class="timeline-box mt-4">
                              <h5 class="resume-experience-title">{"Recent work:"}</h5>
                              <div class="jobster-candidate-timeline">
                               { for recent_work }
                            </div>
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
