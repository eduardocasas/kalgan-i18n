# kalgan-i18n

A translation tool that retrieves the messages stored in yaml files used by Kalgan Framework.

## Examples

This is the yaml file to be used in the following tests:
```yaml
## tests/en/messages.yaml

hello:
  world: Hello World!
  somebody: Hello {user}!
```
```rust
use kalgan_i18n::Messages;

let messages: Messages = Messages::new("tests");
```
```rust
assert_eq!(messages.trans("en", "hello.world", HashMap::new()), "Hello World!");
```
```rust
let mut parameters = HashMap::new();
parameters.insert("user", "John".to_string());
assert_eq!(messages.trans("en", "hello.somebody", parameters), "Hello John!");
```
## Documentation

For further information please visit:

* [Official Kalgan Site](https://kalgan.eduardocasas.com)
* [API Documentation on docs.rs](https://docs.rs/crate/kalgan-i18n/latest)


## License

This crate is licensed under either of the following licenses:

* [MIT License](https://choosealicense.com/licenses/mit/)
* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
