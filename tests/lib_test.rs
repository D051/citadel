// -------------------------
// License
// -------------------------

// Copyright 2024 Dominik Schweigler

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// -------------------------
// Imports
// -------------------------

use citadel;
use citadel::Form;
use citadel::Header;
use citadel::Mode;

// -------------------------
// Tests
// -------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_form_conversion() {
        assert_eq!(Form::from_u8(0), Form::Introduce);
        assert_eq!(Form::from_u8(1), Form::Promotion);
        assert_eq!(Form::from_u8(2), Form::Handshake);
        assert_eq!(Form::from_u8(3), Form::Heartbeat);

        assert_eq!(Form::Introduce.into_u8(), 0);
        assert_eq!(Form::Promotion.into_u8(), 1);
        assert_eq!(Form::Handshake.into_u8(), 2);
        assert_eq!(Form::Heartbeat.into_u8(), 3);
    }

    #[test]
    fn test_mode_conversion() {
        assert_eq!(Mode::from_u8(0), Mode::Secure);
        assert_eq!(Mode::from_u8(1), Mode::Unsafe);
        assert_eq!(Mode::from_u8(2), Mode::Signed);

        assert_eq!(Mode::Secure.into_u8(), 0);
        assert_eq!(Mode::Unsafe.into_u8(), 1);
        assert_eq!(Mode::Signed.into_u8(), 2);
    }

    #[test]
    fn test_header_manipulation() {
        let mut header: [u8; 2] = [0; 2];

        // Test form manipulation

        header.set_form(Form::Introduce);
        assert_eq!(header.get_form(), Form::Introduce);

        header.set_form(Form::Promotion);
        assert_eq!(header.get_form(), Form::Promotion);

        header.set_form(Form::Handshake);
        assert_eq!(header.get_form(), Form::Handshake);

        header.set_form(Form::Heartbeat);
        assert_eq!(header.get_form(), Form::Heartbeat);

        // Test mode manipulation

        header.set_mode(Mode::Secure);
        assert_eq!(header.get_mode(), Mode::Secure);

        header.set_mode(Mode::Signed);
        assert_eq!(header.get_mode(), Mode::Signed);

        header.set_mode(Mode::Unsafe);
        assert_eq!(header.get_mode(), Mode::Unsafe);

        // Test overwrite correctness

        header.set_form(Form::Introduce);
        assert_eq!(header.get_form(), Form::Introduce);

        header.set_mode(Mode::Unsafe);
        assert_eq!(header.get_mode(), Mode::Unsafe);
    }
}
