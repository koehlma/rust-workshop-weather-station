//! Platform code of the Pico W firmware in this workspace.
//!
//! [`watchdog`] feeds the watchdog that the bootloader starts, and [`logger`]
//! sends logs to the USB serial port. With the `nexigon` feature, [`nexigon`]
//! connects a device to Nexigon Hub.

#![no_std]

pub mod logger;
#[cfg(feature = "nexigon")]
pub mod nexigon;
pub mod watchdog;
