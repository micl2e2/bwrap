// Copyright (C) 2023 Michael Lee
//
// Licensed under the MIT License <LICENSE-MIT or
// https://opensource.org/license/mit> or the GNU General Public License,
// Version 3.0 or any later version <LICENSE-GPL or
// https://www.gnu.org/licenses/gpl-3.0.txt>, at your option.
//
// This file may not be copied, modified, or distributed except except in
// compliance with either of the licenses.
//

use crate::Result;
use crate::WrapError;
use crate::WrapStyle;
use crate::Wrapper;

use std::{borrow::Cow, string::String, vec::Vec};

pub struct EasyWrapper<'bf> {
    max_width: usize,
    before: &'bf str,
    after: Vec<u8>,
}

impl<'bf> EasyWrapper<'bf> {
    pub fn new(before: &'bf str, max_width: usize) -> Result<Self> {
        let bf_len = before.len();

        let mut after = Vec::<u8>::new();
        after.resize(bf_len, 0); // assuming default style, i.e. nobreak

        if max_width == 0 {
            return Err(WrapError::InvalidWidth);
        }

        Ok(EasyWrapper {
            max_width,
            before,
            after,
        })
    }

    pub fn wrap<'af>(&'af mut self) -> Result<Cow<'af, str>> {
        let mut wrapper = Wrapper::new(self.before, self.max_width, &mut self.after)?;

        let af_len = wrapper.wrap_no_break()?;

        Ok(String::from_utf8_lossy(&self.after[..af_len]))
    }

    pub fn wrap_use_style<'af>(&'af mut self, style: WrapStyle) -> Result<Cow<'af, str>> {
        match style {
            WrapStyle::NoBreak => self.wrap(),

            WrapStyle::NoBreakAppend(append_what, enl_pref) => {
                let bf_len = self.before.len();
                let max_width = self.max_width;

                self.after.resize(bf_len + bf_len * append_what.len(), 0);

                let mut wrapper = Wrapper::new(self.before, self.max_width, &mut self.after)?;
                let af_len = wrapper.wrap_no_break_append(append_what, enl_pref)?;

                Ok(String::from_utf8_lossy(&self.after[..af_len]))
            }

            WrapStyle::MayBreak => {
                let bf_len = self.before.len();
                let max_width = self.max_width;

                self.after.resize(bf_len + bf_len / max_width, 0);

                let mut wrapper = Wrapper::new(self.before, self.max_width, &mut self.after)?;
                let af_len = wrapper.wrap_may_break()?;

                Ok(String::from_utf8_lossy(&self.after[..af_len]))
            }

            WrapStyle::MayBreakPrepend(prepend_what) => {
                let bf_len = self.before.len();
                let max_width = self.max_width;

                self.after.resize(
                    bf_len + bf_len / max_width + prepend_what.len() * (bf_len / max_width),
                    0,
                );

                let mut wrapper = Wrapper::new(self.before, self.max_width, &mut self.after)?;
                let af_len = wrapper.wrap_may_break_prepend(prepend_what)?;

                Ok(String::from_utf8_lossy(&self.after[..af_len]))
            }

            WrapStyle::MayBreakAppend(append_what) => {
                let bf_len = self.before.len();
                let max_width = self.max_width;

                self.after.resize(
                    bf_len + bf_len / max_width + append_what.len() * (bf_len / max_width),
                    0,
                );

                let mut wrapper = Wrapper::new(self.before, self.max_width, &mut self.after)?;
                let af_len = wrapper.wrap_may_break_append(append_what)?;

                Ok(String::from_utf8_lossy(&self.after[..af_len]))
            }
        }
    }
}
