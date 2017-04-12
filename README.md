DRY blanket implementations for generic traits!
===============================================

Do you have a generic trait? Do you feel that adding a blanket impl for it is unnecessary repetition? Then this is the 
macro for you! This is a macro based on [proc-macro-hack](https://github.com/dtolnay/proc-macro-hack) that helps to 
reduce some of the repetition, which works with any Rust version >= 1.15.0.

The `autoimpl!` macro generates a default blanket impl for a generic trait for all `T` with _the same bounds_ as the 
trait passed into the `autoimpl!` block. 

## Caveats

* This only works for traits with type parameters 
* You can only pass in one trait per `autoimpl!` block
* Only a single type parameter is currently supported 
* Currently, due to a limitation in proc-macro-hack, you can only have one `autoimpl!` block per module.


## Installation

If you're using `Cargo`, just add autoimpl to your `Cargo.toml`:

```toml
[dependencies]
autoimpl = "0.1"
```

## Example

```rust
    #[macro_use] extern crate autoimpl;

    struct Dog {}

    struct Duck {}

    fn main() {
        autoimpl! {
            trait Quack<T> {
                fn say(&self) -> String {
                    "quack".to_string()
                }
            }
        }
        
        let dog = Dog {};
        let duck = Duck {};
        assert_eq!(dog.say(), duck.say());
    }
```
