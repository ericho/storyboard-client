extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

static STORYBOARD_API: &'static str = "https://storyboard.openstack.org/api/v1";

pub mod projects;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
