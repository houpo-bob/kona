#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/kona/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/kona/main/assets/favicon.ico",
    issue_tracker_base_url = "https://github.com/op-rs/kona/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate tracing;

mod task_queue;
pub use task_queue::{
    BuildTask, BuildTaskError, ConsolidateTask, ConsolidateTaskError, Engine, EngineBuildError,
    EngineResetError, EngineTask, EngineTaskError, EngineTaskErrorSeverity, EngineTaskErrors,
    EngineTaskExt, FinalizeTask, FinalizeTaskError, InsertTask, InsertTaskError, SynchronizeTask,
    SynchronizeTaskError,
};

mod attributes;
pub use attributes::{AttributesMatch, AttributesMismatch};

mod client;
pub use client::{EngineClient, EngineClientError};

mod versions;
pub use versions::{EngineForkchoiceVersion, EngineGetPayloadVersion, EngineNewPayloadVersion};

mod state;
pub use state::{EngineState, EngineSyncState, EngineSyncStateUpdate};

mod kinds;
pub use kinds::EngineKind;

mod query;
pub use query::{EngineQueries, EngineQueriesError, EngineQuerySender};

mod metrics;
pub use metrics::Metrics;
