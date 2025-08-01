//! Core [`RollupNode`] service, composing the available [`NodeActor`]s into various modes of
//! operation.
//!
//! [`NodeActor`]: crate::NodeActor

mod core;
pub use core::RollupNodeService;

mod standard;
pub use standard::{RollupNode, RollupNodeBuilder};

mod mode;
pub use mode::{InteropMode, NodeMode};

pub(crate) mod util;
pub(crate) use util::spawn_and_wait;
