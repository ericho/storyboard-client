extern crate storyboard_client;

use storyboard_client::Client;

fn main() {
    let client = Client::new("https://storyboard.openstack.org/api/v1");
    let stories = client.search_stories("stx").unwrap();
    for s in &stories {
        println!("{} - {}", s.id, s.title);
    }
}
