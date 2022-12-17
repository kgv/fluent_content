use fluent::{bundle::FluentBundle, FluentResource};
use fluent_content::prelude::*;
use intl_memoizer::concurrent::IntlLangMemoizer;

fn bundle<T: ToString>(source: T) -> FluentBundle<FluentResource, IntlLangMemoizer> {
    let mut bundle = FluentBundle::new_concurrent(vec![]);
    let resource = FluentResource::try_new(source.to_string()).unwrap();
    bundle.add_resource(resource).unwrap();
    bundle
}

#[test]
fn id() {
    let bundle = bundle("id = Value");

    let content = bundle.content("id");
    assert!(matches!(content, Some(content) if content == "Value"));

    let content = bundle.content("id1");
    assert!(content.is_none());
}

#[test]
fn id_args() {
    let bundle = bundle("id = Value with argument { $arg }");

    let content = bundle.content("id?arg=Argument");
    assert!(
        matches!(content, Some(content) if content == "Value with argument \u{2068}Argument\u{2069}")
    );

    let content = bundle.content("id?arg1=Argument 1");
    assert!(content.is_none());
}

#[test]
fn id_attr() {
    let bundle = bundle("id =\n.attr = Attribute value");

    let content = bundle.content("id.attr");
    assert!(matches!(content, Some(content) if content == "Attribute value"));

    let content = bundle.content("id.attr1");
    assert!(content.is_none());

    let content = bundle.content("id");
    assert!(content.is_none());
}

#[test]
fn id_attr_args() {
    let bundle = bundle("id =\n.attr = Attribute value with argument { $arg }");

    let content = bundle.content("id.attr?arg=Argument");
    assert!(
        matches!(content, Some(content) if content == "Attribute value with argument \u{2068}Argument\u{2069}")
    );

    let content = bundle.content("id.attr?arg1=Argument 1");
    assert!(content.is_none());
}
