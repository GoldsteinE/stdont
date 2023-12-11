//! APIs that feel like they should be in the standard library, but they are not.
//!
//! Also some unstable APIs if they're trivial enough to implement externally.

#![cfg_attr(not(feature = "std"), no_std)]
// lint me harder
#![forbid(non_ascii_idents)]
#![deny(
    future_incompatible,
    keyword_idents,
    elided_lifetimes_in_paths,
    meta_variable_misuse,
    noop_method_call,
    pointer_structural_match,
    unused_lifetimes,
    unused_qualifications,
    unsafe_op_in_unsafe_fn,
    clippy::undocumented_unsafe_blocks,
    clippy::wildcard_dependencies,
    clippy::debug_assert_with_mut_call,
    clippy::empty_line_after_outer_attr,
    clippy::panic,
    clippy::unwrap_used,
    clippy::redundant_field_names,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::useless_let_if_seq
)]
#![warn(clippy::pedantic, missing_docs)]
// not that hard
#![allow(clippy::module_name_repetitions)] // common for extension traits

mod macros;
mod option;
#[cfg(feature = "std")]
mod path;
mod primitive;
mod result;
#[cfg(feature = "alloc")]
mod vec;

// Glob reexports look poorly in the docs
macro_rules! define_prelude {
    ($($(#[$a:meta])* pub use $p:path;)*) => {
        /// Re-exports for stuff that can be safely glob-imported.
        pub mod prelude {
            $($(#[$a])* pub use $p;)*
        }

        $($(#[$a])* pub use $p;)*
    }
}

define_prelude! {
    pub use crate::option::OptionExt;
    pub use crate::primitive::BoolExt;
    pub use crate::result::ResultExt;

    #[cfg(feature = "std")]
    pub use crate::path::PathBufExt;

    #[cfg(feature = "alloc")]
    pub use crate::vec::VecExt;
}
