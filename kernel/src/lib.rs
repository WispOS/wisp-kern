//! This module is the central inclusion point for nearly everything.

// HAHA EVIL >:3c
// NOTE(able): I will flip this flag when my documentation spree is finished.
// #![deny(missing_docs)]
#![no_std]
#![feature(abi_x86_interrupt)]

/// The module containing GlobalDescriptorTable setup.
pub mod gdt;
/// The module defining hardware trait and interfaces and such.
pub mod hardware;
/// The module containing InterruptDescriptorTable setup.
pub mod idt;
/// The module containing Workspace IPC in the form of Mailbox's
pub mod mailbox;
/// The module containing native drivers.
pub mod native_drivers;
/// The module containing native services.
pub mod native_service;

/// The module defining Universe.
pub mod universe;
/// The module defining a programs workspace.
pub mod workspace;

/// Kernel initialization.
pub fn init() {
    idt::init_idt();
}
