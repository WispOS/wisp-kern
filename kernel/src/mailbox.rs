//! Each workspace has a Mailbox where it can recieve messages.

use crate::workspace::WorkspaceID;

/// A kernel facing MailboxMessage
pub struct MailboxMessage {
    from: WorkspaceID,
    to: WorkspaceID,
    // contents: Vec<u8>,
}

/// A MailboxMessage in a suitable format to give to a Workspace.
pub struct MailboxMessageUserFacing;

/// Configuration settings for a Mailbox
pub struct MailboxSettings {
    // pub allow_list: Vec<WorkspaceID>,
    // Specifically ignore these workspaces when the kernel would have to copy data around to avoid Evil:tm: or Buggy:tm: workspaces
    // pub deny_list: Vec<WorkspaceID>,
}

/// A Mailbox acts as the recieving point of a syscall.
///  One Mailbox exists per workspace.
pub struct Mailbox;
