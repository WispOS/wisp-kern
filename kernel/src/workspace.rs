// A workspace is a program and related environment variables.
// A workspace should not be able to directly edit the fields in this struct.

use crate::mailbox::Mailbox;

/// A WorkspaceID uniquely identifiable.
pub type WorkspaceID = u32;

/// A workspace.
pub struct Workspace {
    /// The Workspace local [[Mailbox]].
    pub mailbox: Mailbox,
}
