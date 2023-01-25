pub(super) mod inner;

use crate::hook::inner::{setup_keyboard_hook, setup_mouse_hook, InnerHook};
use std::sync::Arc;

/// Handle to a low-level Windows hook for keyboard and/or mouse events, regardless of application focus.
/// For more details see the HookBuilder. When the handle goes out of scope, then the low-level hook is removed.
/// 
/// Example
/// ```rust
/// # fn main() {
/// # use monke::hook::HookBuilder;
/// {
///     // create low-level hook and store handle in `hook`
///     let hook = HookBuilder::new().with_mouse().build().unwrap();
/// }
/// // hook handle goes out of scope,
/// // underlying low-level hook(s) are unhooked from Windows
/// # }
/// ```
pub struct Hook {
    _keyboard_hook: Option<Arc<InnerHook>>,
    _mouse_hook: Option<Arc<InnerHook>>,
}

impl Hook {
    /// Tries to receive an event from the low-level hook(s) running in the background thread(s).
    /// If there are no events at the moment, will return Err(std::sync::mpsc::Empty):
    /// 
    /// ```rust
    /// # fn main() {
    /// # use monke::hook::HookBuilder;
    /// # use std::sync::mpsc::TryRecvError;
    /// // create low-level hook and store handle in `hook`
    /// let hook = HookBuilder::new().with_mouse().build().unwrap();
    /// // This example definitely can't receive any user input, so the try_recv will fail:
    /// assert!(hook.try_recv().is_err());
    /// assert_eq!(hook.try_recv().err(), Some(TryRecvError::Empty));
    /// # }
    /// ```
    /// 
    /// Hook::try_recv() should be treated as a foundation for more complex processing. 
    /// For example if one would be intereted in only unique key presses
    /// with timestamps (regardless of how long the key press lasts):
    /// 
    /// ``` rust
    /// # fn main() {
    /// # use monke::hook::{KeyCode, HookBuilder};
    /// # let hook = HookBuilder::new().with_mouse().build().unwrap();
    /// use std::sync::mpsc::channel;
    /// use std::time::Instant;
    /// let (event_sender, _event_receiver) = channel();
    /// while let Ok(event) = hook.try_recv() {
    ///     // Process only "press ups" to find unique key presses,
    ///     // because if a user holds a key, then Windows can emit multiple "key down" events
    ///     if event == KeyCode::Up {
    ///         event_sender.send( (event, Instant::now() ));
    ///     }
    /// }
    /// # }
    /// ```
    pub fn try_recv(&self) -> Result<KeyCode, std::sync::mpsc::TryRecvError> {
        InnerHook::try_recv()
    }
}

/// The only way to build a hook is to use HookBuilder.
/// It is possible to choose what types of hooks are active.
/// Currently only "mouse" and "keyboard" hooks are supported (due to Windows API restrictions).
/// 
/// # Build hook for both mouse and keyboard:
/// ```rust
/// use monke::hook::HookBuilder;
/// fn main() {
///     let hook = HookBuilder::new()
///                 .with_mouse()
///                 .with_keyboard()
///                 .build();
///     assert!(hook.is_some());
/// }
/// ```
/// 
/// # Limitations
/// 
/// At least one hook type has to be specified, otherwise build will fail:
/// ```rust
/// # fn main() {
/// # use monke::hook::HookBuilder;
/// let bad_hook = HookBuilder::new().build();
/// assert!(bad_hook.is_none());
/// # }
/// ```
/// There can be only one hook at the moment, even if we try to create different type:
/// 
/// ```rust
/// # fn main() {
/// # use monke::hook::HookBuilder;
/// let hook = HookBuilder::new()
///             .with_mouse()
///             .build();
/// 
/// assert!(hook.is_some());
/// // Building second hook while the first one is still in scope will fail.
/// // Even if that second hook is keyboard hook:
/// let another_hook = HookBuilder::new().with_keyboard().build();
/// assert!(another_hook.is_none());
/// # }
/// ```
/// 
/// Only after the old hook is dropped, the new one can be created:
/// 
/// ```rust
/// # fn main() {
/// # use monke::hook::HookBuilder;
/// let hook = HookBuilder::new()
///             .with_mouse()
///             .build();
/// 
/// assert!(hook.is_some());
/// // It could go out of scope as well, but let's drop it explicitly:
/// drop(hook);
/// // Since there is no "active" hook at the moment, now we can create another:
/// let another_hook = HookBuilder::new().with_keyboard().build();
/// assert!(another_hook.is_some());
/// # }
/// ```
pub struct HookBuilder {
    mouse: bool,
    keyboard: bool,
}

impl HookBuilder {
    pub fn new() -> Self {
        Self {
            mouse: false,
            keyboard: false,
        }
    }

    /// Instructs builder to spawn a new mouse hook in background thread on HookBuilder::build().
    pub fn with_mouse(mut self) -> Self {
        self.mouse = true;
        self
    }

    /// Instructs builder to spawn a new keyboard hook in background thread on HookBuilder::build().
    pub fn with_keyboard(mut self) -> Self {
        self.keyboard = true;
        self
    }

    /// Builds the requested hooks and returns common handle for them.
    /// If any hooks are active, then the build fails.
    pub fn build(self) -> Option<Hook> {
        if !self.keyboard && !self.mouse {
            return None
        }

        let kb_hook = if self.keyboard {
            setup_keyboard_hook()
        } else {
            None
        };
        let m_hook = if self.mouse {
            setup_mouse_hook()
        } else {
            None
        };

        if kb_hook.is_none() && m_hook.is_none() {
            None
        } else {
            Some(Hook {
                _keyboard_hook: kb_hook,
                _mouse_hook: m_hook,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum KeyCode {
    Down,
    Up,
}

