extern crate storyboard_client;

use storyboard_client::{Client, Story};

fn main() {
    let client = Client::new("https://storyboard.openstack.org/api/v1");

    let projects = client.get_all_projects().unwrap();
    for p in projects {
        println!("{:?}", p);
    }

    let projects = client.search_projects("stx").unwrap();
    for p in projects {
        println!("{:?}", p);
    }

    let groups = client.get_project_groups().unwrap();
    for g in groups {
        println!("{:?}", g);
    }

    let groups = client.get_project_groups_by_name("openstack").unwrap();
    for g in &groups {
        println!("{:?}", g);
    }

    let g = &groups[0];
    println!("==== Using: {}", g.name);
    let projects = client.get_projects_in_group(&g).unwrap();
    for p in projects {
        println!("{:?}", p);
    }

    // let stories = client.search_stories("build").unwrap();
    // for s in &stories {
    //     println!("{:?}", s);
    // }

    let s = Story {
        id: 19,
        ..Default::default()
    };
    println!("Using story id: {}", s.id);
    let tasks = client.get_tasks_in_story(&s).unwrap();
    for t in tasks {
        println!("{:?}", t);
    }

    let tasks = client.search_tasks("tbuilder").unwrap();
    for t in tasks {
        println!("{:?}", t);
    }
}
