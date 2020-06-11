use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    pub id: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub gravatar_email: Option<String>,

    pub baekjoon_id: String,
    pub webpage_url: Option<String>,
    pub github_id: Option<String>,
    pub codeforces_id: Option<String>,
    pub codeup_id: Option<String>,
}
