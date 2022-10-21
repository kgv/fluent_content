# Documentation

[🔼](../README.md) | **English** 🇺🇸 | [Русский 🇷🇺](ru-RU.md)

## Use

```rust
let hello_world = bundle.content("hello-world")?;
```

## Definitions

[***Message***][message] is the basic atomic translation unit for Fluent.

Each *message* has an [***identifier***][identifier].

*Messages* (and [***terms***][term], [***variants***][variant],
[***attributes***][attribute]) store their values as [***patterns***][pattern].

Formated *pattern* are called [***content***][content].

[***Request***][request] is a request to receive *content* specified by the
parameters.

[attribute]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Attribute.html
[content]: https://docs.rs/fluent_content/*/fluent_content/trait.Content.html#tymethod.content
[identifier]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Identifier.html
[message]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Message.html
[pattern]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Pattern.html
[request]: https://docs.rs/fluent_content/*/fluent_content/struct.Request.html
[term]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Term.html
[variant]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Variant.html
