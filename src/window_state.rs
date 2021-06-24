#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowState {
    pub size: (u32, u32),
    pub cursor: (u32, u32),
    pub fps_capped: bool,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            size: (0, 0),
            cursor: (0, 0),
            fps_capped: true,
        }
    }

    pub fn relative_cursor(&self) -> (f32, f32) {
        (self.cursor.0 as f32 / self.size.0 as f32, 1.0 - self.cursor.1 as f32 / self.size.1 as f32)
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self::new()
    }
}
