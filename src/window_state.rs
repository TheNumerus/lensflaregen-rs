#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowState {
    pub size: (u32, u32),
    pub cursor: (u32, u32),
    pub mouse_left_button_pressed: bool,
    pub ui_focused: bool,
    pub fps_capped: bool,
    pub frame_num: u64,
}

impl WindowState {
    pub fn with_size(width: u32, height: u32) -> Self {
        Self {
            size: (width, height),
            cursor: (0, 0),
            fps_capped: true,
            mouse_left_button_pressed: false,
            ui_focused: false,
            frame_num: 0,
        }
    }

    pub fn relative_cursor(&self) -> (f32, f32) {
        (self.cursor.0 as f32 / self.size.0 as f32, 1.0 - self.cursor.1 as f32 / self.size.1 as f32)
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self::with_size(1280, 720)
    }
}
