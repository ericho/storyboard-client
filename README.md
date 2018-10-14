# Storyboard client

[![Linux build status](https://api.travis-ci.org/ericho/storyboard-client.svg)](https://travis-ci.org/ericho/storyboard-client)

This little project is an attempt to write a [storyboard api client](https://docs.openstack.org/infra/storyboard/webapi/v1.html).

Currently only the basic operations are supported and just for querying. In the future we might consider to perform write/delete operations.

## Usage

Add this to your `Cargo.toml`

```toml
[dependencies]
storyboard_client = "*"
```

and this into your code:

```
extern crate storyboard_client
```

## Example

A simple example to search all the stories with the keyword `stx`.

```rust
extern crate storyboard_client;

use storyboard_client::Client;

fn main() {
    let client = Client::new("https://storyboard.openstack.org/api/v1");
    let stories = client.search_stories("stx").unwrap();
    for s in &stories {
        println!("{} - {}", s.id, s.title);
    }
}
```

See more examples in the [examples](examples) folder.

## TODO
- [ ] Add serialization for enums for task or story status.
- [ ] Add `failure` crate.
- [ ] Add search story by tag capability.
- [ ] Change tests to not use network. (yes.. doc tests connects to openstack's API)
