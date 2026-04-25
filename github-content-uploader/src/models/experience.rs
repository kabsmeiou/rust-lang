use serde::{Deserialize, Serialize};
use dialoguer::Input;
use crate::models::{Content, ContentType, Promptable};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
    role: String,
    company: String,
    description: Vec<String>, // bullet points
    company_logo_url: String,
    company_link: String,
}

impl Content for Experience {
    fn content_type(&self) -> ContentType {
        ContentType::Experience
    }
}

impl Promptable for Experience {
    fn prompt() -> Self {
        let role: String = Input::new().with_prompt("Role").interact_text().unwrap();
        let company: String = Input::new().with_prompt("Company").interact_text().unwrap();
        let mut description = Vec::new();
        loop {
            let desc: String = Input::new().with_prompt("Description (leave empty to finish)").allow_empty(true).interact_text().unwrap();
            if desc.is_empty() {
                break;
            }
            description.push(desc);
        }
        let company_logo_url: String = Input::new().with_prompt("Company Logo URL").interact_text().unwrap();
        let company_link: String = Input::new().with_prompt("Company Link").interact_text().unwrap();

        Experience {
            role,
            company,
            description,
            company_logo_url,
            company_link,
        }
    }
}