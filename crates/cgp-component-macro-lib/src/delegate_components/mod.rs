pub mod ast;
pub mod define;
pub mod define_struct;
pub mod delegate;
pub mod impl_delegate;
pub mod macro_gen;
pub mod merge_generics;
// pub mod delegates_to;

pub use define::define_components;
pub use delegate::delegate_components;