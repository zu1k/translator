#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    last_event: u8,
}

impl MouseState {
    pub fn new() -> Self {
        Self { last_event: 0 }
    }

    pub fn down(&mut self) {
        self.last_event = 1
    }

    pub fn moving(&mut self) {
        match self.last_event {
            1 => self.last_event = 2,
            2 => self.last_event = 2,
            _ => self.last_event = 0,
        }
    }

    pub fn release(&mut self) {
        match self.last_event {
            2 => self.last_event = 3,
            _ => self.last_event = 0,
        }
    }

    pub fn is_select(&mut self) -> bool {
        if self.last_event == 3 {
            self.last_event = 0;
            true
        } else {
            false
        }
    }
}
