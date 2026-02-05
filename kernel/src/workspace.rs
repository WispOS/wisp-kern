// A workspace is a program and related environment variables.
// A workspace should not be able to directly edit the fields in this struct.

use crate::mailbox::Mailbox;

pub type WorkspaceID = u32;

pub struct Workspace {
    mailbox: Mailbox,
}
