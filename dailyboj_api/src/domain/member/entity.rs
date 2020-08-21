use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// # Member Schema
/// Entity represents the member of the daily-boj.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[schemars(example = "Member::example")]
pub struct Member {
    /// # Id
    /// Unique ID of the member.
    pub id: String,
    /// # Display Name
    /// Display name of the member.
    pub display_name: String,
    /// # Bio
    /// Bio of the member.
    pub bio: Option<String>,
    /// # Profile Image Url
    /// Profile image url of the member.
    pub profile_image_url: Option<String>,
    /// # Gravatar Email
    /// Gravatar email, that is used for the profile image, of the member.
    pub gravatar_email: Option<String>,

    /// # Baekjoon Id
    /// Baekjoon Online Judge id of the member.
    pub baekjoon_id: String,
    /// # Webpage Url
    /// Url of a web page that belongs to the member.
    pub webpage_url: Option<String>,
    /// # GitHub Id
    /// GitHub id of the member.
    pub github_id: Option<String>,
    /// # CodeForces Id
    /// CodeForces id of the member.
    pub codeforces_id: Option<String>,
    /// # CodeUp Id
    /// CodeUp id of the member.
    pub codeup_id: Option<String>,
}

impl Member {
    pub fn example() -> Self {
        Member {
            id: "ranolp".to_owned(),
            display_name: "Ranol☆P".to_owned(),
            bio: Some("Lazy, but get Better.".to_owned()),
            profile_image_url: None,
            gravatar_email: Some("public.ranolp@gmail.com".to_owned()),
            baekjoon_id: "asdhugh1".to_owned(),
            webpage_url: Some("https://twitter.com/RanolP_777".to_owned()),
            github_id: Some("RanolP".to_owned()),
            codeforces_id: Some("RanolP".to_owned()),
            codeup_id: Some("asdhugh1".to_owned()),
        }
    }
}
