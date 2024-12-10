# Mouse and Keyboard Control Library for Windows

This library provides a set of utilities to programmatically control mouse and keyboard inputs on Windows platforms. It is built using the Windows API ( `windows` crate) and enables seamless interaction with the system's input mechanisms. The library is designed for automation tasks, testing, and other scenarios requiring simulated user input.

## Key Features

1. **Mouse Control:**
   - Move the cursor to a specified screen position ( `set_cursor_pos` ).
   - Perform drag-and-drop operations ( `drag_and_drop` ).
   - Simulate mouse clicks for both left and right buttons ( `click_mouse_button` , `click_mouse_button_left` , `click_mouse_button_right` ).

2. **Keyboard Control:**
   - Simulate key presses ( `press_key` ).

3. **Fine-grained Input Simulation:**
   - Includes customizable delays ( `thread::sleep` ) to mimic real user interaction.

## Example Use Cases

* **GUI Automation**: Simulate user interaction for automated GUI testing.
* **Game Bot Development**: Provide input simulation for gaming automation.
* **Accessibility Tools**: Enable custom tools for improved accessibility and productivity.
