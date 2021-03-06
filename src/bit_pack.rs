// Copyright (c) 2021 Andrew Archibald
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::token::Token::{self, *};

pub struct BitLexer<'a> {
    src: &'a [u8],
    i: usize,
    bit: u8,
}

impl<'a> BitLexer<'a> {
    #[inline]
    pub fn new<B: AsRef<[u8]>>(src: &'a B) -> Self {
        BitLexer {
            src: src.as_ref(),
            i: 0,
            bit: 7,
        }
    }

    pub fn next_bit(&mut self) -> Option<bool> {
        if self.i >= self.src.len() {
            return None;
        }
        let b = self.src[self.i];
        // Ignore trailing zeros on the last byte
        if self.i + 1 == self.src.len() && b << (7 - self.bit) == 0 {
            return None;
        }
        let bit = b & (1 << self.bit) != 0;
        if self.bit == 0 {
            self.bit = 7;
            self.i += 1;
        } else {
            self.bit -= 1;
        }
        Some(bit)
    }
}

impl<'a> Iterator for BitLexer<'a> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Token> {
        match self.next_bit() {
            Some(true) => match self.next_bit() {
                Some(true) => Some(L),
                Some(false) => Some(T),
                None => None, // marker bit
            },
            Some(false) => Some(S),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::test::TUTORIAL_TOKENS;

    #[test]
    fn bit_lex_tutorial() {
        let src = [
            0b00010111, 0b10001000, 0b00101011, 0b01101011, 0b01000010, 0b01001110, 0b11000001,
            0b01110000, 0b01100001, 0b00101011, 0b10001011, 0b10001000, 0b01001011, 0b11011010,
            0b00001010, 0b11110001, 0b00001001, 0b01101111, 0b11111100,
        ];
        let tokens = BitLexer::new(&src).collect::<Vec<_>>();
        assert_eq!(tokens, TUTORIAL_TOKENS);
    }
}
