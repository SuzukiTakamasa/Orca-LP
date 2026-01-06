use crate::{NotificationData, NotificationError};

/// LINE notification component
pub struct LineNotifier {
    channel_access_token: String,
    user_id: String,
    client: reqwest::Client,
}

impl LineNotifier {
    pub fn new(channel_access_token: String, user_id: String) -> Self {
        Self {
            channel_access_token,
            user_id,
            client: reqwest::Client::new(),
        }
    }

    /// Send notification for repositioning events
    pub async fn send_reposition_notification(
        &self,
        data: &NotificationData,
    ) -> Result<(), NotificationError> {
        // Implementation will be added in later tasks
        todo!("Implementation pending")
    }

    /// Send notification for daily yield collection
    pub async fn send_daily_yield_notification(
        &self,
        data: &NotificationData,
    ) -> Result<(), NotificationError> {
        // Implementation will be added in later tasks
        todo!("Implementation pending")
    }
}