//! Metrics for the Managed Mode RPC client.

/// Container for metrics.
#[derive(Debug, Clone)]
pub(super) struct Metrics;

impl Metrics {
    // --- Metric Names ---
    /// Identifier for the counter of successful RPC requests. Labels: `method`.
    pub(crate) const MANAGED_NODE_RPC_REQUESTS_SUCCESS_TOTAL: &'static str =
        "managed_node_rpc_requests_success_total";
    /// Identifier for the counter of failed RPC requests. Labels: `method`.
    pub(crate) const MANAGED_NODE_RPC_REQUESTS_ERROR_TOTAL: &'static str =
        "managed_node_rpc_requests_error_total";
    /// Identifier for the histogram of RPC request durations. Labels: `method`.
    pub(crate) const MANAGED_NODE_RPC_REQUEST_DURATION_SECONDS: &'static str =
        "managed_node_rpc_request_duration_seconds";

    // --- RPC Method Names (for zeroing) ---
    /// Lists all managed mode RPC methods here to ensure they are pre-registered.
    const RPC_METHODS: [&'static str; 14] = [
        "chain_id",
        "subscribe_events",
        "fetch_receipts",
        "output_v0_at_timestamp",
        "pending_output_v0_at_timestamp",
        "l2_block_ref_by_timestamp",
        "block_ref_by_number",
        "reset_pre_interop",
        "reset",
        "invalidate_block",
        "provide_l1",
        "update_finalized",
        "update_cross_unsafe",
        "update_cross_safe",
    ];

    /// Initializes metrics for the Supervisor RPC service.
    ///
    /// This does two things:
    /// * Describes various metrics.
    /// * Initializes metrics with their labels to 0 so they can be queried immediately.
    pub(crate) fn init(node: &str) {
        Self::describe();
        Self::zero(node);
    }

    /// Describes metrics used in the Supervisor RPC service.
    fn describe() {
        metrics::describe_counter!(
            Self::MANAGED_NODE_RPC_REQUESTS_SUCCESS_TOTAL,
            metrics::Unit::Count,
            "Total number of successful RPC requests processed by the managed mode client"
        );
        metrics::describe_counter!(
            Self::MANAGED_NODE_RPC_REQUESTS_ERROR_TOTAL,
            metrics::Unit::Count,
            "Total number of failed RPC requests processed by the managed mode client"
        );
        metrics::describe_histogram!(
            Self::MANAGED_NODE_RPC_REQUEST_DURATION_SECONDS,
            metrics::Unit::Seconds,
            "Duration of RPC requests processed by the managed mode client"
        );
    }

    /// Initializes metrics with their labels to `0` so they appear in Prometheus from the start.
    fn zero(node: &str) {
        for method_name in Self::RPC_METHODS.iter() {
            metrics::counter!(
                Self::MANAGED_NODE_RPC_REQUESTS_SUCCESS_TOTAL,
                "method" => *method_name,
                "node" => node.to_string()
            )
            .increment(0);
            metrics::counter!(
                Self::MANAGED_NODE_RPC_REQUESTS_ERROR_TOTAL,
                "method" => *method_name,
                "node" => node.to_string()
            )
            .increment(0);
            metrics::histogram!(
                Self::MANAGED_NODE_RPC_REQUEST_DURATION_SECONDS,
                "method" => *method_name,
                "node" => node.to_string()
            )
            .record(0.0); // Record a zero value to ensure the label combination is present
        }
    }
}
