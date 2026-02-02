/*
(driver
    :reset sampl-driver-reset
    ;; This function does need to return.
    ;; if not the kernel will restart the driver.
    ;; Use (@supervisor-spawn ())
    :on-start sampl-driver-start
    )

(sampl-driver-reset (print "Driver Reset."))
(sampl-driver-start (print "Driver Started."))
(sampl-driver-reset (print "Driver Reset."))
*/

#![no_std]
mod hardware;

pub type WorkspaceID = u32;

pub struct DesktopWorkspace {
    // Wether or not to allow total display access to a workspace.
    // This is akin to fullscreen. Use Meta+Esc to reset this back to desktop control.
    lock_display_ownership: bool,
}
