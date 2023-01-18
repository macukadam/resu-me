use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

const STYLE: &str = include_str!("css/resume4.css");

#[styled_component(Resume4)]
pub fn resume4() -> Html {
    let stylesheet = Style::new(STYLE).unwrap();
    let (state, _) = use_store::<GlobalState>();
    let img = format!("data:image/png;base64,{}", state.image_data);
    let mailto = format!("mailto: {}", state.email);
    let phoneto = format!("tel: {}", state.phone);
    let websiteto = format!("tel: {}", state.website);

    let skills = state.skills.iter().map(|skill| {
        let proficiency = format!("{}%", skill.proficiency);
        html!(
            <div class="item">
                <h3 class="level-title">{&skill.skill}</h3>
                <div class="progress level-bar">
                    <div class="progress-bar theme-progress-bar" role="progressbar" style="width: 99%" aria-valuenow={proficiency} aria-valuemin="0" aria-valuemax="100"></div>
                </div>
            </div>
        )
    });

    let education = state.education.iter().map(|education| html!(
        <div class="item">
            <h4 class="degree">{&education.position}</h4>
            <h5 class="meta">{&education.explanation}</h5>
                <div class="time">{"2016 - 2018"}</div>
        </div>
    ));

    let experience = state.work_experience.iter().map(|experience| {
        html!(
            <div class="item">
                <div class="meta">
                    <div class="upper-row">
                        <h3 class="job-title">{&experience.position}</h3>
                        <div class="time">{"2022 - Present"}</div>
                    </div>
                    <div class="company">{"Startup Hubs, San Francisco"}</div>
                </div>
                <div class="details">
                    <p>{&experience.explanation}</p>
                </div>
            </div>
        )
    });

    html! {
        <div class={stylesheet}>
            <div class="wrapper mt-lg-5">
                <div class="sidebar-wrapper">
                    <div class="profile-container">
                        <img class="img-fluid" alt="Profile Photo" src={img}/>
                        <h1 class="name">{"Alan Doe"}</h1>
                        <h3 class="tagline">{"Full Stack Developer"}</h3>
                    </div>

                    <div class="contact-container container-block">
                        <ul class="list-unstyled contact-list">
                            <li class="email"><i class="fa-solid fa-envelope"></i><a href={mailto}>{&state.email}</a></li>
                            <li class="phone"><i class="fa-solid fa-phone"></i><a href={phoneto}>{&state.phone}</a></li>
                            <li class="website"><i class="fa-solid fa-globe"></i><a href={websiteto} target="_blank">{&state.website}</a></li>
                            <li class="linkedin"><i class="fa-brands fa-linkedin-in"></i><a href="#" target="_blank">{"linkedin.com/"}</a></li>
                            <li class="github"><i class="fa-brands fa-github"></i><a href="#" target="_blank">{"github.com/username"}</a></li>
                            <li class="twitter"><i class="fa-brands fa-twitter"></i><a href="https://twitter.com/3rdwave_themes" target="_blank">{"@twittername"}</a></li>
                        </ul>
                    </div>
                    <div class="education-container container-block">
                        <h2 class="container-block-title">{"Education"}</h2>
                        { for education }
                    </div>
                    <div class="languages-container container-block">
                        <h2 class="container-block-title">{"Languages"}</h2>
                        <ul class="list-unstyled interests-list">
                            <li>{"English "}<span class="lang-desc">{"(Native)"}</span></li>
                            <li>{"French "}<span class="lang-desc">{"(Professional)"}</span></li>
                            <li>{"Spanish "}<span class="lang-desc">{"(Professional)"}</span></li>
                        </ul>
                    </div>
                    <div class="interests-container container-block">
                        <h2 class="container-block-title">{"Interests"}</h2>
                        <ul class="list-unstyled interests-list">
                            <li>{"Climbing"}</li>
                            <li>{"Snowboarding"}</li>
                            <li>{"Cooking"}</li>
                        </ul>
                    </div>
                </div>
                <div class="main-wrapper">
                    <section class="section summary-section">
                        <h2 class="section-title"><span class="icon-holder"><i class="fa-solid fa-user"></i></span>{"Career Profile"}</h2>
                        <div class="summary">
                            <p>{"Summarise your career here lorem ipsum dolor sit amet, consectetuer adipiscing elit. You can "}<a href="" target="_blank">{"download this free resume/CV template here"}</a>{". Aenean commodo ligula eget dolor aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu."}</p>
                        </div>
                    </section>
                    <section class="section experiences-section">
                        <h2 class="section-title"><span class="icon-holder"><i class="fa-solid fa-briefcase"></i></span>{"Experiences"}</h2>
                        { for experience }
                    </section>
                    <section class="section projects-section">
                        <h2 class="section-title"><span class="icon-holder"><i class="fa-solid fa-archive"></i></span>{"Projects"}</h2>
                        <div class="intro">
                            <p>{"You can list your side projects or open source libraries in this section. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum et ligula in nunc bibendum fringilla a eu lectus."}</p>
                        </div>
                        <div class="item">
                            <span class="project-title"><a href="https://themes.3rdwavemedia.com/bootstrap-templates/startup/coderpro-bootstrap-5-startup-template-for-software-projects/" target="_blank">{"CoderPro"}</a></span>{" - "}<span class="project-tagline">{"A responsive website template designed to help developers launch their software projects."}</span>
                        </div>
                        <div class="item">
                            <span class="project-title"><a href="https://themes.3rdwavemedia.com/bootstrap-templates/startup/launch-bootstrap-5-template-for-saas-businesses/" target="_blank">{"Launch"}</a></span>{" - "}<span class="project-tagline">{"A responsive website template designed to help startups promote their products or services."}</span>
                        </div>
                        <div class="item">
                            <span class="project-title"><a href="https://themes.3rdwavemedia.com/bootstrap-templates/resume/devcard-bootstrap-5-vcard-portfolio-template-for-software-developers/" target="_blank">{"DevCard"}</a></span>{" - "}<span class="project-tagline">{"A portfolio website template designed for software developers."}</span>
                        </div>
                        <div class="item">
                            <span class="project-title"><a href="https://themes.3rdwavemedia.com/bootstrap-templates/startup/bootstrap-template-for-mobile-apps-nova-pro/" target="_blank">{"Nova Pro"}</a></span>{" - "}<span class="project-tagline">{"A responsive Bootstrap theme designed to help app developers promote their mobile apps"}</span>
                        </div>
                        <div class="item">
                            <span class="project-title"><a href="http://themes.3rdwavemedia.com/website-templates/responsive-bootstrap-theme-web-development-agencies-devstudio/" target="_blank">{"DevStudio"}</a></span>{" - "}
                            <span class="project-tagline">{"A responsive website template designed to help web developers/designers market their services. "}</span>
                        </div>
                    </section>

                    <section class="skills-section section">
                        <h2 class="section-title"><span class="icon-holder"><i class="fa-solid fa-rocket"></i></span>{"Skills &amp; Proficiency"}</h2>
                        <div class="skillset">
                            { for skills }
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}
