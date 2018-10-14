extern crate reqwest;

use std::default::Default;
use Error;
use Client;

use chrono::{DateTime, Utc};

pub const DEFAULT_PROJ_LIMIT: i32 = 100;

/// Represents a project type in the storyboard API
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    /// Means that storyboard will try to create branches automatically
    pub autocreate_branch: Option<bool>,
    /// Details about the project.
    pub description: Option<String>,
    /// Tells if the project is active or has been deleted.
    pub is_active: bool,
    /// The project unique name.
    pub name: String,
    /// The repo link to the project.
    pub repo_url: Option<String>,
}

/// Represents a group of projects
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectGroup {
    /// The unique ID for the project group.
    pub id: i32,
    /// The unique name for the project group.
    pub name: String,
    /// The full name of the project group.
    pub title: String,
    /// Date when this project group was created.
    pub created_at: DateTime<Utc>,
}

impl Default for ProjectGroup {
    fn default() -> ProjectGroup {
        ProjectGroup {
            id: 0,
            name: String::new(),
            title: String::new(),
            created_at: Utc::now(),
        }
    }
}

impl Client {

    /// Retrieves all the projects from the storyboard API.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// use storyboard_client::{Client, Error};
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<Error>> {
    ///     let client = Client::new("https://storyboard.openstack.org/api/v1");
    ///     let projects = client.get_all_projects()?;
    ///     assert_ne!(projects.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_all_projects(&self)
                            -> Result<Vec<Project>, Error> {
        let url = format!("{}/projects?limit={}", self.uri, DEFAULT_PROJ_LIMIT);
        let projects: Vec<Project> = self.fetch_url(&url)?;
        Ok(projects)
    }

    /// Search projects with the given search string
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// use storyboard_client::{Client, Error};
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<Error>> {
    ///     let client = Client::new("https://storyboard.openstack.org/api/v1");
    ///     let projects = client.search_projects("stx")?;
    ///     assert_ne!(projects.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn search_projects(&self, s: &str)
                           -> Result<Vec<Project>, Error> {
        let url = format!("{}/projects/search?q={}", self.uri, s);
        let projects: Vec<Project> = self.fetch_url(&url)?;
        Ok(projects)
    }

    /// Get all registered project groups.
    pub fn get_project_groups(&self)
                              -> Result<Vec<ProjectGroup>, Error> {
        let url = format!("{}/project_groups", self.uri);
        let groups: Vec<ProjectGroup> = self.fetch_url(&url)?;
        Ok(groups)
    }

    /// Get a project group by it's name.
    pub fn get_project_groups_by_name(&self, name: &str)
                                      -> Result<Vec<ProjectGroup>, Error> {
        let url = format!("{}/project_groups?name={}", self.uri, name);
        let groups: Vec<ProjectGroup> = self.fetch_url(&url)?;
        Ok(groups)
    }

    /// Retrieves all projects in a project group.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// use storyboard_client::{Client, Error, ProjectGroup};
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<Error>> {
    ///     let client = Client::new("https://storyboard.openstack.org/api/v1");
    ///     let group = ProjectGroup { id: 86, ..Default::default() };
    ///     let projects = client.get_projects_in_group(&group)?;
    ///     assert_ne!(projects.len(), 0);
    ///     Ok(())
    /// }
    /// ```

    pub fn get_projects_in_group(&self, g: &ProjectGroup)
                                 -> Result<Vec<Project>, Error> {
        let url = format!("{}/project_groups/{}/projects", self.uri, g.id);
        let projects: Vec<Project> = self.fetch_url(&url)?;
        Ok(projects)
    }

}
