use std::fmt::Display;

use crate::ffi;
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct H3Cell(pub(crate) u64);

impl H3Cell {
    pub unsafe fn new_unchecked(n: u64) -> Self {
        Self(n)
    }

    fn as_bit_view(&self) -> &BitSlice<u64, Msb0> {
        self.0.view_bits()
    }

    fn as_bit_view_mut(&mut self) -> &mut BitSlice<u64, Msb0> {
        self.0.view_bits_mut()
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn get_parent(&self, res: u32) -> Result<Self> {
        if !(0..=self.get_resolution()).contains(&res) {
            return Err(anyhow!("invalid parent resolution"));
        }

        if res == 0 {
            let mut parent = Self(0u64);
            let base = self.get_base_cell();
            parent.reset();
            parent.set_base_cell(base)?;
            return Ok(parent);
        }

        let mut parent = self.clone();

        if res == self.get_resolution() {
            return Ok(self.clone())
        }

        parent.set_resolution(res)?;

        for digit in res+1..=self.get_resolution() {
            parent.set_digit_unused(digit)?
        }

        Ok(parent)
    }

    const PENTAGONS: [u32; 12] = [4, 14, 24, 38, 49, 58, 63, 72, 83, 97, 107, 117];

    pub fn is_base_cell_pentagon(&self) -> bool {
        match Self::PENTAGONS.binary_search(&self.get_base_cell()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn reset(&mut self) {
        self.set_reserved1();
        self.set_reserved2();
        self.set_cell_mode();

        let _ = self.set_resolution(0);
        let _ = self.set_base_cell(0);

        for i in 1..=15 {
            let _ = self.set_digit_unused(i);
        }
    }

    fn set_reserved1(&mut self) {
        *self.as_bit_view_mut().get_mut(0).unwrap() = false;
    }

    fn set_reserved2(&mut self) {
        self.as_bit_view_mut()[5..8].store_be(0);
    }

    fn set_cell_mode(&mut self) {
        self.as_bit_view_mut()[1..5].store_be(1);
    }

    pub fn get_base_cell(&self) -> u32 {
        self.as_bit_view()[12..19].load_be()
    }

    pub fn set_base_cell(&mut self, bc: u32) -> Result<()> {
        if !(0..=121).contains(&bc) {
            return Err(anyhow!("invalid base cell"));
        };
        self.as_bit_view_mut()[12..19].store_be(bc);
        Ok(())
    }

    pub fn get_resolution(&self) -> u32 {
        self.as_bit_view()[8..12].load_be()
    }

    pub fn set_resolution(&mut self, res: u32) -> Result<()> {
        if !(0..=15).contains(&res) {
            return Err(anyhow!("invalid resolution"));
        };
        self.as_bit_view_mut()[8..12].store_be(res);
        Ok(())
    }

    pub fn get_digit(&self, digit: u32) -> Result<u32> {
        if !(1..=15).contains(&digit) {
            return Err(anyhow!("invalid digit"));
        };
        let digit_val =
            self.as_bit_view()[((16 + 3 * digit) as usize)..((19 + 3 * digit) as usize)].load_be();
        Ok(digit_val)
    }

    pub fn set_digit(&mut self, digit: u32, val: u32) -> Result<()> {
        if !(1..=15).contains(&digit) {
            return Err(anyhow!("invalid digit"));
        };
        if !(0..=6).contains(&val) {
            return Err(anyhow!("invalid value"));
        };
        if self.is_base_cell_pentagon() && val == 1 {
            return Err(anyhow!(
                "invalid value (1 is invalid for a cell with a pentagon base cell)"
            ));
        }
        self.as_bit_view_mut()[((16 + 3 * digit) as usize)..((19 + 3 * digit) as usize)]
            .store_be(val);
        Ok(())
    }

    pub fn set_digit_unused(&mut self, digit: u32) -> Result<()> {
        if !(1..=15).contains(&digit) {
            return Err(anyhow!("invalid digit"));
        };
        self.as_bit_view_mut()[((16 + 3 * digit) as usize)..((19 + 3 * digit) as usize)]
            .store_be(7);
        Ok(())
    }

    pub fn generate_random(res: u32) -> Result<Self> {
        let mut s = Self(0u64);

        let mut rng = rand::thread_rng();

        s.reset();

        s.set_resolution(res)?;
        let bc: u32 = rng.gen_range(0..=121);
        s.set_base_cell(bc).unwrap();

        for digit in 1..=res {
            let val = rng.gen_range(0..=6);
            let _ = s.set_digit(digit, val);
        }

        if s.is_base_cell_pentagon() {
            for digit in 1..=res {
                let val = rng.gen_range(0..=5);
                let _ = s.set_digit(
                    digit,
                    match val {
                        1 => 6,
                        n => n,
                    },
                );
            }
        } else {
            for digit in 1..=res {
                let val = rng.gen_range(0..=6);
                let _ = s.set_digit(digit, val);
            }
        }

        Ok(s)
    }

    pub fn generate_from_parent(parent: H3Cell, res: u32) -> Result<Self> {
        let mut s = Self(0u64);
        let parent_res = parent.get_resolution();

        if parent_res > res {
            return Err(anyhow!("parent res larger than child res"));
        };

        *s.as_bit_view_mut() |= &(*parent.as_bit_view());

        if parent_res == res {
            return Ok(parent);
        }

        s.set_resolution(res)?;

        let mut rng = rand::thread_rng();

        if s.is_base_cell_pentagon() {
            for digit in (parent_res + 1)..=res {
                let val = rng.gen_range(0..=5);
                let _ = s.set_digit(
                    digit,
                    match val {
                        1 => 6,
                        n => n,
                    },
                );
            }
        } else {
            for digit in (parent_res + 1)..=res {
                let val = rng.gen_range(0..=6);
                let _ = s.set_digit(digit, val);
            }
        }

        Ok(s)
    }

    pub fn pretty_print(&self) -> String {
        let mut string = String::new();
        string += "H3Cell {\n";
        string += &format!("\tresolution: {}\n", self.get_resolution());
        string += &format!("\tbase cell: {}\n", self.get_base_cell());
        for i in 1..=15 {
            string += &format!("\tdigit {i}: {:?}\n", self.get_digit(i));
        }
        string += "}\n";
        string
    }

    pub fn valid_digits(&self) -> Vec<u32> {
        (1..=self.get_resolution())
            .map(|digit| self.get_digit(digit).unwrap())
            .collect::<Vec<_>>()
    }

    fn zeroed(&mut self) {
        self.as_bit_view_mut().store_be(0u64);
    }
}

impl Display for H3Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "H3Cell({:x})", self.0)
    }
}

impl Into<u64> for H3Cell {
    fn into(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for H3Cell {
    type Error = TryFromError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if ffi::is_h3_valid(value) {
            Ok(Self(value))
        } else {
            Err(TryFromError::InvalidH3Index)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TryFromError {
    #[error("H3 index is not valid")]
    InvalidH3Index,
}
