//! Utilities for calculating
//! [Representation Independent Hashes](https://internetcomputer.org/docs/current/references/ic-interface-spec/#hash-of-map)
//! of [crate::Request] and [crate::Response] objects.

mod request_hash;
pub use request_hash::*;

mod response_hash;
pub use response_hash::*;
