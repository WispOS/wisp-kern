use crate::{
    hardware::{HardwareHandle, interfaces::HWInterface},
    workspace::WorkspaceID,
};
/// pointer to the vga memory space
pub struct VgaBufferWrapper {
    ptr: *mut u8,
}
impl VgaBufferWrapper {
    /// Crete a new handle to vga
    pub fn new_handle() -> Self {
        let vga = 0xB8000 as *mut u8;

        Self { ptr: vga }
    }
}
/// VGA Modes
pub enum VgaMode {
    /// Default is 80x25 in our current qemu config
    Default,
}

/// A handle to the VGA buffer
/// Use this to track modes, which Workspace is using the VGAHandle
pub struct VgaHandle {
    /// vga mode.
    pub mode: VgaMode,
    /// Which workspace owns the vga handle atm
    // NOTE: Probably not needed or can be done somehow else. Possibly with a
    pub workspace_owner: WorkspaceID,
    /// the vga buffer.
    pub buffer: VgaBufferWrapper,
}
impl VgaHandle {
    ///  Nre vga handle
    pub fn new(vga_mode: VgaMode) -> Self {
        let vga_handle = VgaHandle {
            mode: VgaMode::Default,
            // 0 -> means that there is no workspace owner.
            workspace_owner: 0,
            buffer: VgaBufferWrapper::new_handle(),
        };
        vga_handle
    }
}

impl HardwareHandle for VgaHandle {
    fn resettable(&mut self) -> bool {
        true
    }

    // fn interfaces(&mut self) -> alloc::vec::Vec<HWInterface> {
    //     todo!()
    // }

    // fn types(&mut self) -> alloc::vec::Vec<crate::hardware::device_types::HWTypes> {
    //     todo!()
    // }
}

// Used for A Graphical interface into the vga mode.
// impl DisplayIO for VgaHandle {
//     fn refresh(&mut self) {}
// }

/// Translate an XY coordinate pair into an index
pub fn xy_to_idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
