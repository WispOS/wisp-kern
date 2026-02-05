use crate::{
    hardware::{HardwareHandle, interfaces::HWInterface},
    sysio::displayio::DisplayIO,
    workspace::WorkspaceID,
};

pub struct VgaBufferWrapper {
    ptr: *mut u8,
}
impl VgaBufferWrapper {
    pub fn new_handle() -> Self {
        let vga = 0xB8000 as *mut u8;

        Self { ptr: vga }
    }
}

pub enum VgaMode {
    Default,
}

// A handle to the VGA buffer
// Use this to track modes, which Workspace is using the VGAHandle
pub struct VgaHandle {
    pub mode: VgaMode,
    pub workspace_owner: WorkspaceID,
    pub buffer: VgaBufferWrapper,
}
impl VgaHandle {
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
impl DisplayIO for VgaHandle {
    fn refresh(&mut self) {
        let vga = self.buffer.ptr;

        for y in 0..25 {
            for x in 0..80 {
                let offset = xy_to_idx(x, y, 80);
                unsafe {
                    *vga.offset(offset as isize * 2) = b' ' as u8;
                    *vga.offset(offset as isize * 2 + 1) = 0x0f;
                }
            }
        }
    }
}

pub fn xy_to_idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
