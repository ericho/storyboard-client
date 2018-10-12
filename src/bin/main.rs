extern crate storyboard_client;

use storyboard_client::projects;

fn main() {
    let projects = projects::get_all().unwrap();
    for p in projects {
        println!("{:?}", p);
    }

    let projects = projects::search("stx").unwrap();
    for p in projects {
        println!("{:?}", p);
    }

    let groups = projects::get_groups().unwrap();
    for g in groups {
        println!("{:?}", g);
    }

    let groups = projects::get_groups_by_name("openstack").unwrap();
    for g in groups {
        println!("{:?}", g);
    }
}
