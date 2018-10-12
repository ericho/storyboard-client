use crate::STORYBOARD_API;
use chrono::{DateTime, Utc};

const DEFAULT_PROJ_LIMIT: i32 = 100;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    autocreate_branch: Option<bool>,
    description: Option<String>,
    is_active: bool,
    name: String,
    repo_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectGroup {
    id: i32,
    name: String,
    title: String,
    created_at: DateTime<Utc>,
}

pub fn get_all() -> Result<Vec<Project>, Box<std::error::Error>> {
    let url = format!("{}/projects?limit={}", STORYBOARD_API, DEFAULT_PROJ_LIMIT);
    let projects: Vec<Project> = reqwest::get(&url)?.json()?;
    Ok(projects)
}

pub fn search(s: &str) -> Result<Vec<Project>, Box<std::error::Error>> {
    let url = format!("{}/projects/search?q={}", STORYBOARD_API, s);
    let projects: Vec<Project> = reqwest::get(&url)?.json()?;
    Ok(projects)
}

pub fn get_groups() -> Result<Vec<ProjectGroup>, Box<std::error::Error>> {
    let url = format!("{}/project_groups", STORYBOARD_API);
    let groups: Vec<ProjectGroup> = reqwest::get(&url)?.json()?;
    Ok(groups)
}

pub fn get_groups_by_name(name: &str) -> Result<Vec<ProjectGroup>, Box<std::error::Error>> {
    let url = format!("{}/project_groups?name={}", STORYBOARD_API, name);
    let groups: Vec<ProjectGroup> = reqwest::get(&url)?.json()?;
    Ok(groups)
}
