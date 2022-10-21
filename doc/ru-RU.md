# Документация

[🔼](../README.md) | [English 🇺🇸](en-US.md) | **Русский** 🇷🇺

## Использование

```rust
let hello_world = bundle.content("hello-world")?;
```

## Определения

[***Cообщение***][message] является атомарной единицей перевода во Fluent.

Каждое *сообщение* имеет [***идентификатор***][identifier].

*Сообщения* (как и [***термы***][term], [***варианты***][variant],
[***аттрибуты***][attribute]) хранят свои значения в виде
[***паттернов***][pattern].

Форматированный *паттерн* называется [***контентом***][content].

[***Запрос***][request] представляет собой запрос на получение соответствующего
заданным параметрам *контента*.

[attribute]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Attribute.html
[content]: https://docs.rs/fluent_content/*/fluent_content/trait.Content.html#tymethod.content
[identifier]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Identifier.html
[message]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Message.html
[pattern]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Pattern.html
[request]: https://docs.rs/fluent_content/*/fluent_content/struct.Request.html
[term]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Term.html
[variant]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Variant.html
