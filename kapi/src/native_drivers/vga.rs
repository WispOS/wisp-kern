pub struct VgaBufferWrapper {}

pub enum VgaMode {
    Default,
}

// A handle to the VGA buffer
// Use this to track modes, which Workspace is using the VGAHandle
pub struct VgaHandle {
    mode: VgaMode,
    workspace_owner: WorkspaceID,
}

impl HardwareHandle for VgaHandle {
    fn resettable() -> bool {
        true
    }
}
