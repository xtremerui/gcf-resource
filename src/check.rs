extern crate gcf_resource;

use gcf_resource::Source;
use gcf_resource::Version;

#[derive(Debug)]
struct CheckRequest {
    pub source: Source,
    pub version: Version,
}

fn main() {
    let req = CheckRequest {
        source: Source {
            json_key: String::from("test"),
        },
        version: Version {
            version: String::from("some-version"),
        },
    };

    println!("Hello {:?}", req);
}
