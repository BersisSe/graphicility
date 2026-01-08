use std::collections::HashSet;

use winit::event::{MouseButton, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use winit_input_helper::WinitInputHelper;

use crate::Graphics;

pub struct Input {
    pub(crate) helper: WinitInputHelper,
    mouse_logical: Option<(f32,f32)>,
    
    // Key Tracking
    keys_pressed: HashSet<PhysicalKey>,
    keys_released: HashSet<PhysicalKey>,
    keys_held: HashSet<PhysicalKey>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            helper: WinitInputHelper::new(),
            mouse_logical: None,
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            keys_held: HashSet::new(),
        }
    }

    /// Process a keyboard event directly
    pub(crate) fn process_key_event(&mut self, event: &KeyEvent) {
        let physical_key = event.physical_key;
        
        match event.state {
            winit::event::ElementState::Pressed => {
                if !self.keys_held.contains(&physical_key) {
                    self.keys_pressed.insert(physical_key);
                }
                self.keys_held.insert(physical_key);
            }
            winit::event::ElementState::Released => {
                self.keys_held.remove(&physical_key);
                self.keys_released.insert(physical_key);
            }
        }
    }

    /// Clear transient input state (pressed/released) for the next frame
    pub(crate) fn reset_transient_state(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    // ? Keyboard

    /// Returns true while the key is held down
    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keys_held.iter().any(|&k| k == PhysicalKey::Code(key))
    }

    /// Returns true only on the frame the key was pressed
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.iter().any(|&k| k == PhysicalKey::Code(key))
    }

    /// Returns true only on the frame the key was released
    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keys_released.iter().any(|&k| k == PhysicalKey::Code(key))
    }

    // ? Mouse
    pub(crate) fn update_mouse_mapping(&mut self, gfx: &Graphics) {
        self.mouse_logical = self.helper.cursor().map(|(mx, my)| {
            let (lw, lh) = gfx.logical_size();
            let (ww, wh) = gfx.window_size();

            let x = mx * lw as f32 / ww as f32;
            let y = my * lh as f32 / wh as f32;
            return (x , y);
        });
    }
    /// Returns true if the MouseButton is down in the current frame.
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.helper.mouse_held(button)
    }
    /// Returns true if the MouseButton is pressed.
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.helper.mouse_pressed(button)
    }
    /// Returns true if the MouseButton is released in the current frame.
    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.helper.mouse_released(button)
    }
    /// Returns the current Mouse position as `f32 Tuple` containg X&Y cordinates. <br>
    /// While window is unfocused it will return `None`
    /// _Note : Mouse Positions need to be precise thats why we didn't use Vec2 here!_
    pub fn mouse_pos(&self) -> Option<(f32, f32)> {
        self.mouse_logical
    }

    // ? Window Related
    /// Returns the window Size after a resize event.
    pub fn window_resized(&self) -> Option<(u32, u32)> {
        self.helper
            .window_resized()
            .map(|size| (size.width, size.height))
    }
    
    pub fn window_close_requested(&self) -> bool {
        self.helper.close_requested()
    }
}
