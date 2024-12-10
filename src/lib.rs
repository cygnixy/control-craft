use std::thread;
use std::time::Duration;
use windows::{
    Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::SetCursorPos,
};

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
}

/// Moves the mouse cursor to the specified screen coordinates (x, y).
///
/// # Parameters
/// - `x`: The x-coordinate on the screen.
/// - `y`: The y-coordinate on the screen.
///
/// # Usage
/// Useful for UI automation, moving the cursor to a specific location on the screen.
pub fn set_cursor_pos(x: i32, y: i32) {
    unsafe {
        let _ = SetCursorPos(x, y);
    }
}

/// Simulates a drag-and-drop operation from the current cursor position
/// to the specified screen coordinates (x, y).
///
/// # Parameters
/// - `x`: The x-coordinate of the drop location.
/// - `y`: The y-coordinate of the drop location.
///
/// # Behavior
/// - Presses and holds the left mouse button.
/// - Moves the cursor to the specified coordinates.
/// - Releases the left mouse button after a delay.
pub fn drag_and_drop(x: i32, y: i32) {
    unsafe {
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        thread::sleep(Duration::from_millis(100));

        let _ = SetCursorPos(x, y);
        thread::sleep(Duration::from_millis(500));

        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}

/// Simulates a mouse button click (left or right).
///
/// # Parameters
/// - `button`: Specifies which mouse button to click (`MouseButton::Left` or `MouseButton::Right`).
///
/// # Behavior
/// - Presses and releases the specified mouse button with a small delay.
pub fn click_mouse_button(button: MouseButton) {
    unsafe {
        match button {
            MouseButton::Left => {
                mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                thread::sleep(Duration::from_millis(100));
                mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
            }
            MouseButton::Right => {
                mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                thread::sleep(Duration::from_millis(100));
                mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
            }
        }
    }
}

/// Simulates a left mouse button click.
///
/// # Behavior
/// - Presses and releases the left mouse button with a small delay.
/// - Equivalent to calling `click_mouse_button(MouseButton::Left)`.
pub fn click_mouse_button_left() {
    unsafe {
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        thread::sleep(Duration::from_millis(100));
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}

/// Simulates a right mouse button click.
///
/// # Behavior
/// - Presses and releases the right mouse button with a small delay.
/// - Equivalent to calling `click_mouse_button(MouseButton::Right)`.
pub fn click_mouse_button_right() {
    unsafe {
        mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
        thread::sleep(Duration::from_millis(100));
        mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
    }
}

/// Simulates pressing and releasing a keyboard key.
///
/// # Parameters
/// - `key`: The virtual-key code of the key to press (e.g., `0x41` for 'A').
///
/// # Behavior
/// - Presses the specified key.
/// - Releases the specified key immediately after pressing.
pub fn press_key(key: u8) {
    unsafe {
        keybd_event(key, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(key, 0, KEYBD_EVENT_FLAGS(2), 0);
    }
}
