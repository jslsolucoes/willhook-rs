//! # What this crate provides
//! 
//! This Windows-only crate provides safe and correct means to listen for keyboard and mouse events regardless of application focus.
//! The application can be CLI or with a Window.
//! 
//! Under the hood the crate leverages the **WI**ndows **L**ow-**L**evel **HOOK**s.
//! You can read more about that topic on [MSDN](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-hooks?redirectedfrom=MSDN).
//! 
//! ### What this crate does NOT provide
//! 
//! This crate is intended for "read-only" access to hooks. It does not support injecting input events or altering them.
//! 
//! # Warning: The currenct state
//! 
//! Currently it supports mouse and keyboard actions in almost full extent, see [hook::event] module for details.
//! 
//! The design goals for this crate are to be: correct, misuse-proof and fail-proof.
//! 
//! There are tests, but keep in mind that the crate is "young".
//! 
//! *It is highly recommended to at least quickly review the code before using this crate for anything more then hobby projects.*
//! 
//! # How it works
//! 
//! In short, there are a few handy functions to request a hook: [keyboard_hook], [mouse_hook] and [willhook].
//! When called they:
//! - start background thread(s) for each low-level hook, and in that thread(s):
//!     - register a mouse and/or keyboard low-level hook(s)
//!     - start Windows message queue and wait for the message to end execution
//! - create, if were not created already, the channels for passing events to "client" thread
//! - return the handle to the underlying low-level hooks as [hook::Hook]
//! 
//! When the [hook::Hook] goes out of scope, the underlying resources supporting low-level hooks are dropped:
//! - each of the background threads is properly joined
//! - each of the underlying low-level hooks is unhooked from the Windows Kernel
//! 
//! When the [hook::Hook] is active (in scope / not dropped). 
//! Then one can receive recorded [hook::event::InputEvent]s via [hook::Hook::try_recv].
//! It works similiarly to [std::sync::mpsc::Receiver::try_recv].
//! 
//! Note: at the moment the channels for passing events between threads are not "freed" when [hook::Hook] is dropped. 
//!     This means that if not all events were consumed before unhooking, then next hook may receive "old" input events.
//!     That is, input events recorded from before it's creation (or re-creation, so to speak).
//! 
//! # Alternatives:
//! 




pub mod hook;

pub use hook::Hook;
use hook::HookBuilder;

/// Return the Keyboard Hook handle. For more details see [Hook] and [HookBuilder]
pub fn keyboard_hook() -> Option<Hook> {
    HookBuilder::new().with_keyboard().build()
}

/// Return the Mouse Hook handle. For more details see [Hook] and [HookBuilder]
pub fn mouse_hook() -> Option<Hook> {
    HookBuilder::new().with_mouse().build()
}

/// Return the handle for both mouse and keyboard hook. For more details see [Hook] and [HookBuilder]
pub fn willhook() -> Option<Hook> {
    HookBuilder::new().with_keyboard().with_mouse().build()
}