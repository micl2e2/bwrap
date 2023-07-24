// Copyright (C) 2023 Michael Lee <micl2e2@proton.me>
//
// Licensed under the MIT License <LICENSE-MIT or
// https://opensource.org/license/mit> or the GNU General Public License,
// Version 3.0 or any later version <LICENSE-GPL or
// https://www.gnu.org/licenses/gpl-3.0.txt>, at your option.
//
// This file may not be copied, modified, or distributed except except in
// compliance with either of the licenses.
//

use crate::ExistNlPref;
use crate::Result;
use crate::WrapError;
use crate::WrapStyle;
use crate::Wrapper;

use std::{borrow::Cow, string::String, vec::Vec};

use unicode_width::UnicodeWidthStr;

///
/// A type for the actual wrapping tasks.
///
/// Note that libstd is in use for automatic memory management.
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
        self.wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))
    }

    pub fn wrap_use_style<'af>(&'af mut self, style: WrapStyle) -> Result<Cow<'af, str>> {
        match style {
            WrapStyle::NoBrk(append_what, enl_pref) => {
                let bf_len = self.before.len();
                let max_width = self.max_width;

                // FIXME: unnecessary resize after new
                self.after.resize(
                    bf_len
                        + if let Some(v) = append_what {
                            bf_len * v.len()
                        } else {
                            0
                        },
                    0,
                );

                let mut wrapper = Wrapper::new(self.before, self.max_width, &mut self.after)?;
                let af_len = wrapper.internal_wrap_nobrk(append_what, enl_pref)?;

                Ok(String::from_utf8_lossy(&self.after[..af_len]))
            }

            WrapStyle::MayBrk(prepend_what, append_what) => {
                let bf_len = self.before.len();
                let max_width = self.max_width;

                let prepend_str = if prepend_what.is_none() {
                    ""
                } else {
                    prepend_what.expect("bug")
                };
                let append_str = if append_what.is_none() {
                    ""
                } else {
                    append_what.expect("bug")
                };
                self.after.resize(
                    bf_len
                        + if prepend_str.len() == 0 && append_str.len() == 0 {
                            let bf_width = UnicodeWidthStr::width(self.before);
                            bf_width / max_width
                        } else {
                            bf_len * 1 + bf_len * prepend_str.len() + bf_len * append_str.len()
                        },
                    0,
                );

                let mut wrapper = Wrapper::new(self.before, self.max_width, &mut self.after)?;
                let af_len = wrapper.internal_wrap_maybrk(prepend_what, append_what)?;

                Ok(String::from_utf8_lossy(&self.after[..af_len]))
            }
        }
    }
}
