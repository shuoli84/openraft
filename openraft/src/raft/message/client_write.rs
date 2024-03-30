use std::fmt;
use std::fmt::Debug;

use crate::display_ext::DisplayOptionExt;
use crate::LogId;
use crate::Membership;
use crate::RaftTypeConfig;

/// The response to a client-request.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(bound = "C::R: crate::AppDataResponse")
)]
pub struct ClientWriteResponse<C: RaftTypeConfig> {
    /// The id of the log that is applied.
    pub log_id: LogId<C::NodeId>,

    /// Application specific response data.
    pub data: C::R,

    /// If the log entry is a change-membership entry.
    pub membership: Option<Membership<C>>,
}

impl<C: RaftTypeConfig> Debug for ClientWriteResponse<C>
where C::R: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientWriteResponse")
            .field("log_id", &self.log_id)
            .field("data", &self.data)
            .field("membership", &self.membership)
            .finish()
    }
}

impl<C> fmt::Display for ClientWriteResponse<C>
where C: RaftTypeConfig
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClientWriteResponse{{log_id:{}, membership:{}}}",
            self.log_id,
            self.membership.display()
        )
    }
}
