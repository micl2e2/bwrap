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

use unicode_width::UnicodeWidthStr;

use crate::Result;
use crate::WrapError;

use crate::auxbuf::ByteSlcBuf;
use crate::auxbuf::ExisSpcBuf;
use crate::auxbuf::HypoNlBuf;
use crate::auxbuf::HypoNlKind;

///
/// The wrapping style used by [`Wrapper`].
///
/// Bwrap categorizes the user input into two categories:
///
/// 1. space-sensitive
/// 2. space-insensitive
///
/// **"space-sensitive"** suits for the languages that depend on ASCII
/// SPACE to delimit words, such as English, Ukrainian, Greek, etc.
/// **"space-insensitive"** suits for otherwise languages, such as Chinese,
/// Japanese, Thai, etc.
pub enum WrapStyle<'a> {
    ///
    /// Wrapping text will **never** break the original semantics. This is true
    /// for those "space-sensitive" languages.
    ///
    /// If the first value is not `None`, it will be appended to all newly
    /// inserted newlines. The second value instructs the wrap how to deal
    /// with all existing newlines.
    NoBrk(Option<&'a str>, ExistNlPref),

    ///
    /// Wrapping text **may** break the original semantics. For example, the
    /// wrapping `We need an example` with 15-width limit results in
    ///
    /// ```ignored
    /// We need an exam
    /// ple
    /// ```
    ///
    /// If the first value is not `None`, it will be prepended to all newly
    /// inserted newlines. If the second value is not `None`, it will be
    /// appended to all newly inserted newlines.
    MayBrk(Option<&'a str>, Option<&'a str>),
}

///
/// Preference for existing newlines.
#[derive(PartialEq)]
pub enum ExistNlPref {
    ///
    /// Trim all ASCII SPACEs following each existing newline character.
    TrimTrailSpc,

    ///
    /// Keep all ASCII SPACEs following each existing newline character.
    KeepTrailSpc,
}

///
/// A type for the actual wrapping tasks.
///
/// Note that this requires manual memory management.
pub struct Wrapper<'a, 'b> {
    max_width: usize,
    before: &'a str,
    after: &'b mut [u8],
}

impl<'bf, 'af> Wrapper<'bf, 'af> {
    ///
    /// Initialize an Wrapper instance.
    ///
    /// # Errors
    /// If output buffer size is insufficient to hold the final output bytes.
    pub fn new(before: &'bf str, max_width: usize, after: &'af mut [u8]) -> Result<Self> {
        let bf_len = before.len();
        let af_len = after.len();

        if bf_len != 0 && af_len == 0 {
            return Err(WrapError::InsufficentBufferSize(bf_len, af_len, 0));
        }

        if af_len < bf_len {
            return Err(WrapError::InsufficentBufferSize(bf_len, af_len, bf_len));
        }

        if max_width == 0 {
            return Err(WrapError::InvalidWidth);
        }

        Ok(Wrapper {
            before,
            max_width,
            after,
        })
    }

    ///
    /// A convient alias for
    /// `wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc)`.
    pub fn wrap(&mut self) -> Result<usize> {
        self.wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))
    }

    ///
    /// Perform wrapping tasks.
    ///
    /// Note that this method will mutate the output buffer.
    ///
    /// # Errors
    /// If output buffer size is insufficient to hold the final output bytes.
    pub fn wrap_use_style(&mut self, style: WrapStyle) -> Result<usize> {
        match style {
            WrapStyle::NoBrk(append_what, enl_pref) => {
                self.internal_wrap_nobrk(append_what, enl_pref)
            }
            WrapStyle::MayBrk(prepend_what, append_what) => {
                self.internal_wrap_maybrk(prepend_what, append_what)
            }
        }
    }

    fn is_af_buf_suffice(&self, af_len_atleast: usize) -> Result<()> {
        let bf_len = self.before.as_bytes().len();
        let af_len = self.after.len();
        if self.after.len() < af_len_atleast {
            Err(WrapError::InsufficentBufferSize(
                bf_len,
                af_len,
                af_len_atleast,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn internal_wrap_nobrk(
        &mut self,
        append_str: Option<&str>,
        enl_pref: ExistNlPref,
    ) -> Result<usize> {
        use ExistNlPref::TrimTrailSpc;
        use HypoNlKind::{Existing, NewOne};

        let bf_bytes = self.before.as_bytes();
        let bf_len = bf_bytes.len();
        let max_width = self.max_width;

        let append_str = if append_str.is_none() {
            ""
        } else {
            append_str.expect("bug")
        };

        if max_width < UnicodeWidthStr::width(append_str) {
            return Err(WrapError::InvalidWidth);
        }

        if bf_len == 0 {
            return Ok(0);
        }
        self.is_af_buf_suffice(bf_len + bf_len * append_str.len())?; // FIXME: can be less space

        let mut backing_buf_hypo_nls = (usize::MAX, usize::MAX, HypoNlKind::NewOne);
        let mut backing_buf_exis_spcs = (usize::MAX, usize::MAX);

        let mut buf_after = ByteSlcBuf::new(self.after);
        let mut buf_hypo_nls = HypoNlBuf::new(&mut backing_buf_hypo_nls);
        let mut buf_exis_spcs = ExisSpcBuf::new(&mut backing_buf_exis_spcs);
        let mut len_appended = 0usize;

        let append_bytes = append_str.as_bytes();
        let append_len = append_bytes.len();
        let mut width_seen = 0usize;
        let mut width_buried = 0usize;
        let mut pi = 0;
        buf_after.push(bf_bytes[0]);

        let mut try_trim = false;

        for ci in 1..bf_len + 1 {
            let is_last_cp = ci == bf_len;

            if is_last_cp
                || (!is_last_cp && (bf_bytes[ci] < 0b10000000 || bf_bytes[ci] >= 0b11000000))
            {
                let width_just_seen = UnicodeWidthStr::width(&self.before[pi..ci]);
                width_seen += width_just_seen;

                if buf_hypo_nls.len() == 0 {
                    if width_seen > max_width && buf_exis_spcs.len() > 0 {
                        let most_recent_spc = buf_exis_spcs.last();
                        let (spc_i, width_bf_spc_i) = most_recent_spc;
                        buf_after.replace(*spc_i - width_buried, 0xa);
                        buf_after.push_many_at(append_bytes, *spc_i - width_buried + 1);
                        len_appended += append_len;
                        buf_hypo_nls.push((*spc_i, *width_bf_spc_i, NewOne));
                        try_trim = true;
                    }
                } else {
                    let most_recent_nl = buf_hypo_nls.last();
                    let (nl_i, width_bf_nl_i, nlk) = most_recent_nl;
                    let curline_width = match nlk {
                        NewOne => width_seen - width_bf_nl_i - 1,
                        Existing => width_seen - width_bf_nl_i,
                    };
                    if curline_width > max_width && buf_exis_spcs.len() > 0 {
                        let most_recent_spc = buf_exis_spcs.last();
                        let (spc_i, width_bf_spc_i) = most_recent_spc;
                        if spc_i > nl_i {
                            buf_after.replace(*spc_i - width_buried + len_appended, 0xa);
                            buf_after.push_many_at(
                                append_bytes,
                                *spc_i - width_buried + len_appended + 1,
                            );
                            len_appended += append_len;
                            buf_hypo_nls.push((*spc_i, *width_bf_spc_i, NewOne));
                            try_trim = true;
                        }
                    }
                }

                pi = ci;
            }

            if !is_last_cp {
                if try_trim
                    && buf_after.last_many_equal(1 + append_len, 0xa, append_bytes)
                    && bf_bytes[ci] == 0x20
                {
                    width_seen -= 1;
                    width_buried += 1;
                } else if enl_pref == TrimTrailSpc
                    && buf_after.last() == 0xa
                    && bf_bytes[ci] == 0x20
                {
                    width_seen -= 1;
                    width_buried += 1;
                } else {
                    try_trim = false;

                    buf_after.push(bf_bytes[ci]);
                    if bf_bytes[ci] == 0xa {
                        buf_hypo_nls.push((ci, width_seen, Existing));
                    }
                    if bf_bytes[ci] == 0x20 {
                        buf_exis_spcs.push((ci, width_seen));
                    }
                }
            }
        }

        Ok(buf_after.len())
    }

    pub(crate) fn internal_wrap_maybrk(
        &mut self,
        prepend_str: Option<&str>,
        append_str: Option<&str>,
    ) -> Result<usize> {
        use HypoNlKind::{Existing, NewOne};

        let bf_bytes = self.before.as_bytes();
        let bf_len = bf_bytes.len();
        let max_width = self.max_width;

        let prepend_str = if prepend_str.is_none() {
            ""
        } else {
            prepend_str.expect("bug")
        };

        let append_str = if append_str.is_none() {
            ""
        } else {
            append_str.expect("bug")
        };

        if bf_len == 0 {
            return Ok(0);
        }
        self.is_af_buf_suffice(
            bf_len
                + if prepend_str.len() == 0 && append_str.len() == 0 {
                    let bf_width = UnicodeWidthStr::width(self.before);
                    bf_width / max_width
                } else {
                    bf_len * 1 + bf_len * prepend_str.len() + bf_len * append_str.len()
                },
        )?; // FIXME: can be less space

        let mut backing_buf_hypo_nls = (usize::MAX, usize::MAX, HypoNlKind::NewOne);
        let mut buf_after = ByteSlcBuf::new(self.after);
        let mut buf_hypo_nls = HypoNlBuf::new(&mut backing_buf_hypo_nls);
        let mut len_prepended = 0usize;
        let mut len_appended = 0usize;

        let prepend_bytes = prepend_str.as_bytes();
        let prepend_len = prepend_bytes.len();
        let bytes_append = append_str.as_bytes();
        let len_append = bytes_append.len();
        let mut width_seen = 0usize;
        let mut pi = 0;
        buf_after.push(bf_bytes[0]);

        for ci in 1..bf_len + 1 {
            let is_last_cp = ci == bf_len;

            if is_last_cp
                || (!is_last_cp && (bf_bytes[ci] < 0b10000000 || bf_bytes[ci] >= 0b11000000))
            {
                let width_just_seen = UnicodeWidthStr::width(&self.before[pi..ci]);
                width_seen += width_just_seen;

                if buf_hypo_nls.len() == 0 {
                    if width_seen == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            buf_after.push_many(prepend_bytes);
                            len_prepended += prepend_len;
                            buf_after.push(0xa);
                            buf_after.push_many(bytes_append);
                            len_appended += len_append;
                            buf_hypo_nls.push((ci, width_seen, NewOne));
                        }
                    } else if width_seen > max_width {
                        buf_after.ppush_within(prepend_bytes, 0xa, pi, ci);
                        buf_after.push_many_at(bytes_append, pi + 1);
                        len_prepended += prepend_len;
                        len_appended += len_append;
                        buf_hypo_nls.push((pi, width_seen - width_just_seen, NewOne));
                    }
                } else {
                    let most_recent_nl = buf_hypo_nls.last();
                    let (_, width_bf_nl_i, _) = most_recent_nl;
                    let curline_width = width_seen - width_bf_nl_i;
                    if curline_width == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            buf_after.push_many(prepend_bytes);
                            len_prepended += prepend_len;
                            buf_after.push(0xa);
                            buf_after.push_many(bytes_append);
                            len_appended += len_append;
                            buf_hypo_nls.push((ci, width_seen, NewOne));
                        }
                    } else if curline_width > max_width {
                        let len_newones = buf_hypo_nls.len_newones();
                        let len_newcomers = len_newones + len_appended + len_prepended;

                        buf_after.ppush_within(
                            prepend_bytes,
                            0xa,
                            pi + len_newcomers,
                            ci + len_newcomers,
                        );
                        len_prepended += prepend_len;
                        buf_after.push_many_at(bytes_append, pi + len_newcomers + 1);
                        len_appended += len_append;

                        buf_hypo_nls.push((pi, width_seen - width_just_seen, NewOne));
                    }
                }

                pi = ci;
            }

            if !is_last_cp {
                buf_after.push(bf_bytes[ci]);
                if bf_bytes[ci] == 0xa {
                    buf_hypo_nls.push((ci, width_seen, Existing));
                }
            }
        }

        Ok(buf_after.len())
    }
} // impl
