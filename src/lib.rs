pub mod bindings;
pub mod components;

use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct GlobalState {
    pub username: String,
    pub job_title: String,
    pub job_description: String,
    pub email: String,
    pub dob: String,
    pub address: String,
    pub gender: Gender,
    pub website: String,
    pub phone: String,
    pub work_experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub recent_work: Vec<RecentWork>,
    pub socials: Vec<Social>,
    pub image_data: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct Experience {
    pub position: String,
    pub explanation: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct Education {
    pub position: String,
    pub explanation: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct RecentWork {
    pub project: String,
    pub explanation: String,
    pub link: String,
}


#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct Skill {
    pub skill: String,
    pub proficiency: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct Social {
    pub name: String,
    pub link: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub enum Gender {
    DontWantToSay,
    Other,
    Female,
    Male,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            username: "Ugurcan Akpulat".to_string(),
            job_title: "Software Engineer".to_string(),
            job_description: "As an experienced software engineer, I’ve worked in creating
products from the ground up and am proficient in
multiple programming languages such as Rust, TypeScript,
Python and Dart. I like experimenting with different
topics and technologies to learn and grow as an engineer.".to_string(),
            email: "ugurcan.akpulat@gmail.com".to_string(),
            website: "https://github.com/macukadam".to_string(),
            phone: "+31642089969".to_string(),
            work_experience: Experience::default_vec(),
            education: Education::default_vec(),
            skills: Skill::default_vec(),
            recent_work: RecentWork::default_vec(),
            socials: Social::default_vec(),
            image_data: "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".to_string(),
            dob: "04.06.1990".to_string(),
            address: "Amsterdam, Netherlands".to_string(),
            gender: Gender::default(),
        }
    }
}

trait DefaultVec {
    fn default_vec() -> Vec<Self>
    where
        Self: Sized;
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::DontWantToSay => "no gender".to_string(),
            Gender::Male => "male".to_string(),
            Gender::Female => "female".to_string(),
            Gender::Other => "other".to_string(),
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Self::Male
    }
}

impl DefaultVec for Experience {
    fn default_vec() -> Vec<Self> {
        vec![
            Self {
                position: "Swishfund - Senior Software Engineer / December 2021 - Present".to_string(),
                explanation: "• Developing and maintaining the credit analysis platform at Swishfund.
• Extending and improving our open-source PIPY packages.
• Implementing deployment and testing pipelines.
• Developing a loan administration system from the ground up.
• Reducing costs significantly for API usages and cloud services by optimizing code.
• Introducing observability and monitoring tools to the company.
(Python / Django / Flask/ AWS / Airflow)"
                    .to_string(),
            },
            Self {
                position: "Eleena - Software Engineer / October 2019 - December 2021".to_string(),
                explanation: "• Developing the back-end system for reading thousands of DLMS/COSEM compatible smart energy meters using the tornado framework, achieving industry-leading reliability and speed.
• Developing customer management and reporting platforms with the Django framework.
• Engaging in full-stack development, DevOps, testing, and designing new products, including proof of concept designs and database management.
• Leading and supporting a small team of engineers.
• Sharing knowledge daily and taking full responsibility for the products.
(Python / Machine Learning/ Tornado)"
                    .to_string(),
            },
            Self {
                position: "Siemens - Software Engineer / October 2017 - October 2019".to_string(),
                explanation: "• Maintaining and enhancing Siemens' general automation platform, the TIA portal.
• Developing new features and internal tools for controllers using C#.
• Implementing and maintaining new features for different PLC prototypes, including the S7300 and S71500 families.
(C# / WPF / TFS)"
                    .to_string(),
            },
            Self {
                position: "Lambda Construction - Software Developer / April 2017 - October 2017".to_string(),
                explanation: "• Designing the ERP system for Lambda.
• Developing the Lambda website and integrating it seamlessly with the ERP system.
(C# / .NET core / JavaScript / VB)"
                    .to_string(),
            },
        ]
    }
}

impl DefaultVec for Education {
    fn default_vec() -> Vec<Self> {
        vec![
            Self {
                position: "MS Software Engineering - Bogazici University, Istanbul / 2016 - 2019".to_string(),
                explanation: "• Pursued a Master's degree in Software Engineering, focusing on advanced software development techniques and project management. Gained in-depth knowledge of algorithms, system architecture, and software lifecycle.".to_string(),
            },
            Self {
                position: "BS Civil Engineering - Dokuz Eylul University, Izmir / 2009 - 2015".to_string(),
                explanation: "• Completed a Bachelor of Science in Civil Engineering, focusing on structural engineering and construction management. Engaged in comprehensive study and practical projects related to infrastructure and building design.".to_string(),
            },
        ]
    }
}

impl DefaultVec for RecentWork {
    fn default_vec() -> Vec<Self> {
        vec![
            Self {
                project: "Ticker".to_string(),
                explanation: "Personal project - An experimental tool for calculating arbitrage opportunities between different crypto pairs and exchanges.".to_string(),
                link: "https://ticker.lol/".to_string(),
            },
            Self {
                project: "RESU-ME".to_string(),
                explanation: "Personal project - A resume creation application developed using Rust and the Yew framework.".to_string(),
                link: "https://github.com/macukadam/resu-me".to_string(),
            },
            Self {
                project: "Dutch Blog".to_string(),
                explanation: "Personal project - A social app for blogging and sharing my language learning process.".to_string(),
                link: "https://dutch-blog-f6775.web.app/".to_string(),
            },
            Self {
                project: "Lingalunga".to_string(),
                explanation: "Personal project - AI story generation and language learning app.".to_string(),
                link: "https://github.com/macukadam/Lingalunga-backend".to_string(),
            },
            Self {
                project: "TWASTER".to_string(),
                explanation: "School project - Earthquake detection by analysing real-time tweets.".to_string(),
                link: "https://github.com/macukadam/TwitterApiWebApp".to_string(),
            },
            Self {
                project: "TOMRIDDLE".to_string(),
                explanation: "School project - A simple semantic typing suggester utilizing WikiData".to_string(),
                link: "https://github.com/macukadam/TomRiddle".to_string(),
            },
            Self {
                project: "PYPI Packages".to_string(),
                explanation: "Open source python packages!".to_string(),
                link: "https://pypi.org/user/macukadam/".to_string(),
            }
        ]
    }
}


impl DefaultVec for Skill {
    fn default_vec() -> Vec<Self> {
        vec![
            Self {
                skill: "Python".to_string(),
                proficiency: "85".to_string(),
            },
            Self {
                skill: "C#".to_string(),
                proficiency: "70".to_string(),
            },
            Self {
                skill: "Javascript".to_string(),
                proficiency: "70".to_string(),
            },
            Self {
                skill: "Typescript".to_string(),
                proficiency: "70".to_string(),
            },
            Self {
                skill: "Rust".to_string(),
                proficiency: "65".to_string(),
            },
            Self {
                skill: "Git".to_string(),
                proficiency: "65".to_string(),
            },
            Self {
                skill: "Lua".to_string(),
                proficiency: "60".to_string(),
            },
            Self {
                skill: "Docker".to_string(),
                proficiency: "60".to_string(),
            },
            Self {
                skill: "Kubernetes".to_string(),
                proficiency: "55".to_string(),
            },
            Self {
                skill: "AWS".to_string(),
                proficiency: "55".to_string(),
            },
        ]
    }
}

impl DefaultVec for Social {
    fn default_vec() -> Vec<Self> {
        vec![
            Self {
                name: "Github".to_string(),
                link: "https://github.com/macukadam".to_string(),
            },
            Self {
                name: "Linkedin".to_string(),
                link: "https://www.linkedin.com/in/ugurcan-akpulat/".to_string(),
            },
            Self {
                name: "Twitter".to_string(),
                link: "https://twitter.com/macukadam".to_string(),
            },
        ]
    }
}
