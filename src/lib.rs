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
    pub website: String,
    pub phone: String,
    pub work_experience: Vec<Experience>,
    pub education: Vec<Experience>,
    pub skills: Vec<Skill>,
    pub images: Vec<String>,
    pub socials: Vec<Social>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct Experience {
    pub position: String,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct Skill {
    pub skill: String,
    pub proficiency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct Social {
    pub name: String,
    pub link: String,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            username: "John Doe".to_string(),
            job_title: "Frontend Engineer".to_string(),
            job_description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed odio lacus,
                    sollicitudin in dolor at, consequat volutpat ante. Integer quis consequat turpis, quis porta orci. Proin
                    tincidunt volutpat faucibus. Suspendisse ac nisl purus suspendisse eleifend interdum orci non pharetra.".to_string(),
            email: "lcollins@email.com".to_string(),
            website: "lcollins@email.com".to_string(),
            phone: "123-456-7890".to_string(),
            work_experience: vec![Experience::default(), Experience::default(), Experience::default()],
            education: vec![Experience::default(), Experience::default()],
            skills: vec![Skill::default(), Skill::default(), Skill::default(), Skill::default(), Skill::default()],
            images: vec!["http://store-images.s-microsoft.com/image/apps.49745.9007199266437737.188b2a07-b170-4fe0-a52a-63f919ad6d32.47320de6-0cfc-4757-a926-0cfcd81b9d65".to_string(),
                        "http://store-images.s-microsoft.com/image/apps.49745.9007199266437737.188b2a07-b170-4fe0-a52a-63f919ad6d32.47320de6-0cfc-4757-a926-0cfcd81b9d65".to_string()],
            socials: vec![Social::default(), Social::default(), Social::default()],
        }
    }
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            position: "Senior Web Developer / Digital Agency 2016-2020".to_string(),
            explanation: "Phasellus et tellus iaculis, interdum augue vel, luctus nulla. Aenean viverra, magna a ultricies
                      elementum, dui mi tristique ligula, non euismod leo mauris ac metus.".to_string(),
        }
    }
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            skill: "HTML".to_string(),
            proficiency: "70".to_string(),
        }
    }
}

impl Default for Social {
    fn default() -> Self {
        Self {
            name: "Github".to_string(),
            link: "www.github.com".to_string(),
        }
    }
}
