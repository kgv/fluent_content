use fluent::{FluentBundle, FluentResource};
use fluent_content::prelude::*;
use unic_langid::langid;

pub fn main() {
    let source = "hello =\n.world = Hello, { $arg } world!".to_string();
    let resource = FluentResource::try_new(source).expect("Failed to parse an FTL string.");
    let mut bundle = FluentBundle::new(vec![langid!("en")]);
    bundle
        .add_resource(resource)
        .expect("Failed to add FTL resources to the bundle.");
    let hello_world = bundle
        .content("hello.world?arg=brave new")
        .expect("Failed to get FTL content.");
    println!("{hello_world}");
}
