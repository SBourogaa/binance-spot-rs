/**
 * Connection state tracker.
 *
 * Maintains the state of active subscriptions for reconnection purposes.
 *
 * # Fields
 * - `active_subscriptions`: List of currently active stream names.
 */
pub(super) struct ConnectionState {
    active_subscriptions: Vec<String>,
}

impl ConnectionState {
    /**
     * Creates a new connection state tracker.
     *
     * # Returns
     * - New ConnectionState instance with no active subscriptions.
     */
    pub fn new() -> Self {
        Self {
            active_subscriptions: Vec::new(),
        }
    }

    /**
     * Adds a subscription to the active list.
     *
     * # Arguments
     * - `stream_name`: Name of the stream that was subscribed to.
     */
    pub fn add_subscription(&mut self, stream_name: String) {
        self.active_subscriptions.push(stream_name);
    }

    /**
     * Removes subscriptions from the active list.
     *
     * # Arguments
     * - `stream_names`: Names of streams to remove from active list.
     */
    pub fn remove_subscriptions(&mut self, stream_names: &[String]) {
        for stream_name in stream_names {
            self.active_subscriptions.retain(|s| s != stream_name);
        }
    }

    /**
     * Gets the list of active subscriptions.
     *
     * # Returns
     * - Reference to the vector of active subscription names.
     */
    pub fn active_subscriptions(&self) -> &Vec<String> {
        &self.active_subscriptions
    }

    /**
     * Checks if there are any active subscriptions.
     *
     * # Returns
     * - `true` if there are active subscriptions, `false` otherwise.
     */
    pub fn has_active_subscriptions(&self) -> bool {
        !self.active_subscriptions.is_empty()
    }

    /**
     * Clears all active subscriptions.
     */
    #[allow(dead_code)]
    pub fn clear_subscriptions(&mut self) {
        self.active_subscriptions.clear();
    }
}