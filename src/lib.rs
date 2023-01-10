pub mod components;
pub mod bindings;

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
    pub image_data: String
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
            image_data: "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".to_string()
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
