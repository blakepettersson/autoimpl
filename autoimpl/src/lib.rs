#[macro_use] extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use] extern crate autoimpl_derive;

#[doc(hidden)]
pub use autoimpl_derive::*;

extern crate proc_macro;

proc_macro_item_decl! {
    /// Generates a default blanket impl for a generic trait that gets passed into
    /// `autoimpl!`, with the same type parameters and type bounds as its trait.
    ///
    /// # Panics
    ///
    /// * If a trait is not a generic trait
    /// * If more than one trait gets passed into an `autoimpl!` block
    /// * If anything other than a trait gets passed into an `autoimpl!` block
    ///
    /// # Example
    ///
    /// ```
    /// #[macro_use] extern crate autoimpl;
    ///
    /// struct Dog {}
    ///
    /// struct Duck {}
    ///
    /// fn main() {
    ///     autoimpl! {
    ///         trait Quack<T> {
    ///             fn say(&self) -> String {
    ///                 "quack".to_string()
    ///             }
    ///         }
    ///     }
    ///
    ///     let dog = Dog {};
    ///     let duck = Duck {};
    ///     assert_eq!(dog.say(), duck.say());
    /// }
    /// ```
    autoimpl! => generate_auto_impl_impl
}
