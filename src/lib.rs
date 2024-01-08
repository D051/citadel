// -------------------------
// License
// -------------------------

// Copyright 2024 Dominik Schweigler

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// -------------------------
// Form Flags
// -------------------------

/// Form Flag definition
#[derive(Debug, PartialEq, Eq)]
pub enum Form {
    Introduce, // introduce task flag
    Promotion, // promotion task flag
    Handshake, // handshake task flag
    Heartbeat, // heartbeat task flag
}

/// Form Flag implementation
impl Form {
    /// Get a form enum from n u8
    pub fn from_u8(n: u8) -> Form {
        match n {
            0 => Form::Introduce,
            1 => Form::Promotion,
            2 => Form::Handshake,
            3 => Form::Heartbeat,
            _ => panic!("invalid form"),
        }
    }
    /// Convert a form enum into an u8
    pub fn into_u8(&self) -> u8 {
        match self {
            Form::Introduce => 0,
            Form::Promotion => 1,
            Form::Handshake => 2,
            Form::Heartbeat => 3,
        }
    }
}

// -------------------------
// Mode Flags
// -------------------------

/// Mode Flag definition
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Secure, // message is signed and encrypted
    Unsafe, // message is not signed nor encrypted
    Signed, // message is signed but not encrypted
}

/// Mode Flag implementation
impl Mode {
    /// Get a mode enum from n u8
    pub fn from_u8(n: u8) -> Mode {
        match n {
            0 => Mode::Secure,
            1 => Mode::Unsafe,
            2 => Mode::Signed,
            _ => panic!("invalid mode"),
        }
    }
    /// Convert a mode enum into an u8
    pub fn into_u8(&self) -> u8 {
        match self {
            Mode::Secure => 0,
            Mode::Unsafe => 1,
            Mode::Signed => 2,
        }
    }
}

// -------------------------
// Header
// -------------------------

/// Header trait definition
pub trait Header {
    /// Set task
    fn set_form(&mut self, form: Form);
    /// Set mode
    fn set_mode(&mut self, mode: Mode);
    // Get form
    fn get_form(&self) -> Form;
    // Get mode
    fn get_mode(&self) -> Mode;
}

/// Header trait implementation
impl Header for [u8; 2] {
    /// Set form
    fn set_form(&mut self, form: Form) {
        // write form to bits 0 & 1 on byte 0
        self[0] = (form.into_u8() << 6) | (self[0] & 0b00111111);
    }
    /// Set mode
    fn set_mode(&mut self, mode: Mode) {
        // write mode to bits 2 & 3 on byte 0
        self[0] = (mode.into_u8() << 4) | (self[0] & 0b11001111);
    }
    // Get form
    fn get_form(&self) -> Form {
        let form: u8 = self[0] >> 6 & 0b00000011;
        Form::from_u8(form)
    }
    // Get mode
    fn get_mode(&self) -> Mode {
        let mode: u8 = self[0] >> 4 & 0b0000011;
        Mode::from_u8(mode)
    }
}
