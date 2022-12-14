use fluent::types::FluentValue;
use fluent_content::Request;

#[test]
fn id() {
    assert!(matches!(Request::from("id"), Request { id: "id", .. }));
}

#[test]
fn id_args() {
    assert!(matches!(
        Request::from("id?key1=value1&key2=2"),
        Request {
            id: "id",
            args: Some(args),
            ..
        } if args.get("key1") == Some(&"value1".into())
            && args.get("key2") == Some(&2.into())
            && args.get("key3").is_none()
    ));
}

#[test]
fn id_arg_none() {
    assert!(matches!(
        Request::from("id?key="),
        Request {
            id: "id",
            args: Some(args),
            ..
        } if matches!(
            args.get("key"),
            Some(FluentValue::None)
        )
    ));
}

#[test]
fn id_arg_error() {
    assert!(matches!(
        Request::from("id?key"),
        Request {
            id: "id",
            args: Some(args),
            ..
        } if matches!(
            args.get("key"),
            Some(FluentValue::Error)
        )
    ));
}

#[test]
fn id_attr() {
    assert!(matches!(
        Request::from("id.attr"),
        Request {
            id: "id",
            attr: Some("attr"),
            ..
        }
    ));
}

#[test]
fn id_attr_args() {
    assert!(matches!(
        Request::from("id.attr?key1=value1&key2=2"),
        Request {
            id: "id",
            attr: Some("attr"),
            args: Some(args),
        } if args.get("key1") == Some(&"value1".into())
            && args.get("key2") == Some(&2.into())
            && args.get("key3").is_none()
    ));
}

#[test]
fn id_attr_arg_none() {
    assert!(matches!(
        Request::from("id.attr?key="),
        Request {
            id: "id",
            attr: Some("attr"),
            args: Some(args),
        } if matches!(
            args.get("key"),
            Some(FluentValue::None)
        )
    ));
}

#[test]
fn id_attr_arg_error() {
    assert!(matches!(
        Request::from("id.attr?key"),
        Request {
            id: "id",
            attr: Some("attr"),
            args: Some(args),
        } if matches!(
            args.get("key"),
            Some(FluentValue::Error)
        )
    ));
}
