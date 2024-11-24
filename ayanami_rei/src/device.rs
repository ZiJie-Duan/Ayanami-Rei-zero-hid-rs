use std::{fs::File, io::Write};

const KEYBOARD_PATH: &str = "/dev/hidg0";
const RELA_MOUSE_PATH: &str = "/dev/hidg1";
const ABSL_MOUSE_PATH: &str = "/dev/hidg2";

pub trait DeviceBuffer: Default {
    // default all data zero
    fn new() -> Self {
        Self::default()
    }
    fn to_vec(&self) -> Vec<u8>;
}

#[derive(Default)]
pub struct KeyboardBuf {
    pub modifier: u8, // Modifier key
    pub reserved: u8, // Reserved byte
    pub keys: [u8; 6], // Key values of regular keys pressed
}

impl DeviceBuffer for KeyboardBuf{
    fn to_vec(&self) -> Vec<u8> {
        let mut v:Vec<u8> = Vec::new();
        v.push(self.modifier);
        v.push(self.reserved);
        self.keys.iter().for_each(|e| v.push(*e));
        v
    }
}

#[derive(Default)]
pub struct RelaMouseBuf {
    pub button_status: u8, // Button status
    pub x_movement: i8,    // Change in X coordinate
    pub y_movement: i8,    // Change in Y coordinate
    pub v_wheel: i8,       // Change in vertical wheel
    pub h_wheel: i8,       // Change in horizontal wheel
}

impl DeviceBuffer for RelaMouseBuf{
    fn to_vec(&self) -> Vec<u8> {
        let mut v:Vec<u8> = Vec::new();
        v.push(self.button_status);
        v.push(self.x_movement);
        v.push(self.y_movement);
        v.push(self.v_wheel);
        v.push(self.h_wheel);
        v
    }
}

#[derive(Default)]
pub struct AbslMouseBuf {
    pub button_status: u8, // Button status
    pub x_position: u16,   // Absolute position of X coordinate
    pub y_position: u16,   // Absolute position of Y coordinate
    pub v_wheel: i8,       // Change in vertical wheel
    pub h_wheel: i8,       // Change in horizontal wheel
}

impl DeviceBuffer for AbslMouseBuf{
    fn to_vec(&self) -> Vec<u8> {
        let mut v:Vec<u8> = Vec::new();
        v.push(self.button_status);
        v.extend(self.x_position.to_le_bytes().iter());
        v.extend(self.y_position.to_le_bytes().iter());
        v.push(self.v_wheel);
        v.push(self.h_wheel);
        v
    }
}

#[derive(Default)]
pub enum HIDBuffer {
    Keyboard(KeyboardBuf),
    RelaMouse(RelaMouseBuf),
    AbslMouse(AbslMouseBuf),
}

impl DeviceBuffer for HIDBuffer {
    fn to_vec(&self) -> Vec<u8> {
        self.to_vec()
    }
}


pub struct Device{
    keyboard:File,
    rela_mouse:File,
    abs_mouse:File,
}

impl Default for Device {
    fn default() -> Self {
        Self { 
            keyboard: File::create(KEYBOARD_PATH)?, 
            rela_mouse: File::create(RELA_MOUSE_PATH)?, 
            abs_mouse: File::create(ABSL_MOUSE_PATH)?,
        }
    }
}

impl Device {
    pub fn new() -> Self{
        Self::default()
    }

    pub fn send(&mut self, buf:HIDBuffer){
        match buf {
            HIDBuffer::Keyboard(buf) => {
                self.keyboard.write_all(&buf)?
            },
            HIDBuffer::RelaMouse(buf) => {
                self.rela_mouse.write_all(&buf)?
            },
            HIDBuffer::AbslMouse(buf) => {
                self.abs_mouse.write_all(&buf)?
            },
        }
    }
}

