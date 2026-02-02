use crate::{
    hardware::{HardwareHandle, interfaces::HWInterface},
    workspace::WorkspaceID,
};

pub struct VgaBufferWrapper {}

pub enum VgaMode {
    Default,
}

// A handle to the VGA buffer
// Use this to track modes, which Workspace is using the VGAHandle
pub struct VgaHandle {
    pub mode: VgaMode,
    pub workspace_owner: WorkspaceID,
}
impl VgaHandle {
    pub fn new(vga_mode: VgaMode) -> Self {
        let vga_handle = VgaHandle {
            mode: VgaMode::Default,
            // 0 -> means that there is no workspace owner.
            workspace_owner: 0,
        };
        vga_handle
    }
}

impl HardwareHandle for VgaHandle {
    fn resettable() -> bool {
        true
    }

    fn interfaces() -> alloc::vec::Vec<HWInterface> {
        todo!()
    }

    fn types() -> alloc::vec::Vec<crate::hardware::device_types::HWTypes> {
        todo!()
    }
}
