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

use unicode_width::UnicodeWidthStr;

use crate::Result;
use crate::WrapError;

use crate::auxbuf::Aux1Buf;
use crate::auxbuf::Aux2Buf;
use crate::auxbuf::Aux2ElemKind;
use crate::auxbuf::Aux3Buf;

pub enum WrapStyle<'a> {
    NoBreak,
    NoBreakAppend(&'a str, ExistNlPref),
    MayBreak,
    MayBreakPrepend(&'a str),
    MayBreakAppend(&'a str),
}

#[derive(PartialEq)]
pub enum ExistNlPref {
    TrimTrailSpc,
    KeepTrailSpc,
}

pub struct Wrapper<'a, 'b> {
    max_width: usize,
    before: &'a str,
    after: &'b mut [u8],
}

impl<'bf, 'af> Wrapper<'bf, 'af> {
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

    pub fn wrap(&mut self) -> Result<usize> {
        self.wrap_no_break()
    }

    pub fn wrap_use_style(&mut self, style: WrapStyle) -> Result<usize> {
        match style {
            WrapStyle::NoBreak => self.wrap_no_break(),
            WrapStyle::NoBreakAppend(append_what, enl_pref) => {
                self.wrap_no_break_append(append_what, enl_pref)
            }
            WrapStyle::MayBreak => self.wrap_may_break(),
            WrapStyle::MayBreakPrepend(prepend_what) => self.wrap_may_break_prepend(prepend_what),
            WrapStyle::MayBreakAppend(append_what) => self.wrap_may_break_append(append_what),
        }
    }

    // private //

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

    pub(crate) fn wrap_no_break(&mut self) -> Result<usize> {
        use Aux2ElemKind::{Abnormal, Normal};

        let bf_bytes = self.before.as_bytes();
        let bf_len = bf_bytes.len();
        let max_width = self.max_width;

        if bf_len == 0 {
            return Ok(0);
        }
        self.is_af_buf_suffice(bf_len)?;

        let mut backing_buf_aux2 = (usize::MAX, usize::MAX, Aux2ElemKind::Normal);
        let mut backing_buf_aux3 = (usize::MAX, usize::MAX);

        let mut buf_aux1 = Aux1Buf::new(self.after);
        let mut buf_aux2 = Aux2Buf::new(&mut backing_buf_aux2);
        let mut buf_aux3 = Aux3Buf::new(&mut backing_buf_aux3);

        let mut width_seen = 0usize;
        let mut width_buried = 0usize;
        let mut pi = 0;
        buf_aux1.push(bf_bytes[0]);

        for ci in 1..bf_len + 1 {
            let is_last_cp = ci == bf_len;

            if is_last_cp
                || (!is_last_cp && (bf_bytes[ci] < 0b10000000 || bf_bytes[ci] >= 0b11000000))
            {
                let width_just_seen = UnicodeWidthStr::width(&self.before[pi..ci]);
                width_seen += width_just_seen;

                if buf_aux2.len() == 0 {
                    if width_seen > max_width && buf_aux3.len() > 0 {
                        let most_recent_aux3 = buf_aux3.last();
                        let (aux3_i, mark_aux3_i) = most_recent_aux3;
                        buf_aux1.replace(*aux3_i - width_buried, 0xa);
                        buf_aux2.push((*aux3_i, *mark_aux3_i, Normal));
                    }
                } else {
                    let most_recent_aux2 = buf_aux2.last();
                    let (aux2_i, mark_aux2_i, elemk) = most_recent_aux2;
                    let cl_width = match elemk {
                        Normal => width_seen - mark_aux2_i - 1,
                        Abnormal => width_seen - mark_aux2_i,
                    };
                    if cl_width > max_width && buf_aux3.len() > 0 {
                        let most_recent_aux3 = buf_aux3.last();
                        let (aux3_i, mark_aux3_i) = most_recent_aux3;
                        if aux3_i > aux2_i {
                            buf_aux1.replace(*aux3_i - width_buried, 0xa);
                            buf_aux2.push((*aux3_i, *mark_aux3_i, Normal));
                        }
                    }
                }

                pi = ci;
            }

            if !is_last_cp {
                if buf_aux1.last() == 0xa && bf_bytes[ci] == 0x20 {
                    width_seen -= 1;
                    width_buried += 1;
                } else {
                    buf_aux1.push(bf_bytes[ci]);
                    if bf_bytes[ci] == 0xa {
                        buf_aux2.push((ci, width_seen, Abnormal));
                    }
                    if bf_bytes[ci] == 0x20 {
                        buf_aux3.push((ci, width_seen));
                    }
                }
            }
        }

        Ok(buf_aux1.len())
    }

    pub(crate) fn wrap_no_break_append(
        &mut self,
        append_str: &str,
        enl_pref: ExistNlPref,
    ) -> Result<usize> {
        use Aux2ElemKind::{Abnormal, Normal};
        use ExistNlPref::TrimTrailSpc;

        let bf_bytes = self.before.as_bytes();
        let bf_len = bf_bytes.len();
        let max_width = self.max_width;

        if max_width < UnicodeWidthStr::width(append_str) {
            return Err(WrapError::InvalidWidth);
        }

        if bf_len == 0 {
            return Ok(0);
        }
        self.is_af_buf_suffice(
            bf_len + bf_len / max_width + append_str.len() * (bf_len / max_width),
        )?;

        let mut backing_buf_aux2 = (usize::MAX, usize::MAX, Aux2ElemKind::Normal);
        let mut backing_buf_aux3 = (usize::MAX, usize::MAX);

        let mut buf_aux1 = Aux1Buf::new(self.after);
        let mut buf_aux2 = Aux2Buf::new(&mut backing_buf_aux2);
        let mut buf_aux3 = Aux3Buf::new(&mut backing_buf_aux3);
        let mut aux4 = 0usize;

        let append_bytes = append_str.as_bytes();
        let append_len = append_bytes.len();
        let mut width_seen = 0usize;
        let mut width_buried = 0usize;
        let mut pi = 0;
        buf_aux1.push(bf_bytes[0]);

        let mut try_trim = false;

        for ci in 1..bf_len + 1 {
            let is_last_cp = ci == bf_len;

            if is_last_cp
                || (!is_last_cp && (bf_bytes[ci] < 0b10000000 || bf_bytes[ci] >= 0b11000000))
            {
                let width_just_seen = UnicodeWidthStr::width(&self.before[pi..ci]);
                width_seen += width_just_seen;

                if buf_aux2.len() == 0 {
                    if width_seen > max_width && buf_aux3.len() > 0 {
                        let most_recent_aux3 = buf_aux3.last();
                        let (aux3_i, mark_aux3_i) = most_recent_aux3;
                        buf_aux1.replace(*aux3_i - width_buried, 0xa);
                        buf_aux1.push_many_at(append_bytes, *aux3_i - width_buried + 1);
                        aux4 += append_len;
                        buf_aux2.push((*aux3_i, *mark_aux3_i, Normal));
                        try_trim = true;
                    }
                } else {
                    let most_recent_aux2 = buf_aux2.last();
                    let (aux2_i, mark_aux2_i, elemk) = most_recent_aux2;
                    let cl_width = match elemk {
                        Normal => width_seen - mark_aux2_i - 1,
                        Abnormal => width_seen - mark_aux2_i,
                    };
                    if cl_width > max_width && buf_aux3.len() > 0 {
                        let most_recent_aux3 = buf_aux3.last();
                        let (aux3_i, mark_aux3_i) = most_recent_aux3;
                        if aux3_i > aux2_i {
                            buf_aux1.replace(*aux3_i - width_buried + aux4, 0xa);
                            buf_aux1.push_many_at(append_bytes, *aux3_i - width_buried + aux4 + 1);
                            aux4 += append_len;
                            buf_aux2.push((*aux3_i, *mark_aux3_i, Normal));
                            try_trim = true;
                        }
                    }
                }

                pi = ci;
            }

            if !is_last_cp {
                if try_trim
                    && buf_aux1.last_many_equal(1 + append_len, 0xa, append_bytes)
                    && bf_bytes[ci] == 0x20
                {
                    width_seen -= 1;
                    width_buried += 1;
                } else if enl_pref == TrimTrailSpc && buf_aux1.last() == 0xa && bf_bytes[ci] == 0x20
                {
                    width_seen -= 1;
                    width_buried += 1;
                } else {
                    try_trim = false;

                    buf_aux1.push(bf_bytes[ci]);
                    if bf_bytes[ci] == 0xa {
                        buf_aux2.push((ci, width_seen, Abnormal));
                    }
                    if bf_bytes[ci] == 0x20 {
                        buf_aux3.push((ci, width_seen));
                    }
                }
            }
        }

        Ok(buf_aux1.len())
    }

    pub(crate) fn wrap_may_break(&mut self) -> Result<usize> {
        use Aux2ElemKind::{Abnormal, Normal};

        let bf_bytes = self.before.as_bytes();
        let bf_len = bf_bytes.len();
        let max_width = self.max_width;

        if bf_len == 0 {
            return Ok(0);
        }
        self.is_af_buf_suffice(bf_len + bf_len / max_width)?;

        let mut backing_buf_aux2 = (usize::MAX, usize::MAX, Aux2ElemKind::Normal);

        let mut buf_aux1 = Aux1Buf::new(self.after);
        let mut buf_aux2 = Aux2Buf::new(&mut backing_buf_aux2);

        let mut width_seen = 0usize;

        let mut pi = 0;
        buf_aux1.push(bf_bytes[0]);

        for ci in 1..bf_len + 1 {
            let is_last_cp = ci == bf_len;

            if is_last_cp
                || (!is_last_cp && (bf_bytes[ci] < 0b10000000 || bf_bytes[ci] >= 0b11000000))
            {
                let width_just_seen = UnicodeWidthStr::width(&self.before[pi..ci]);
                width_seen += width_just_seen;

                if buf_aux2.len() == 0 {
                    if width_seen == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            buf_aux1.push(0xa);
                            buf_aux2.push((ci, width_seen, Normal));
                        }
                    } else if width_seen > max_width {
                        buf_aux1.push_within(0xa, pi, ci);
                        buf_aux2.push((pi, width_seen - width_just_seen, Normal));
                    }
                } else {
                    let most_recent_aux2 = buf_aux2.last();
                    let (_, mark_aux2_i, _) = most_recent_aux2;
                    let cl_width = width_seen - mark_aux2_i;
                    if cl_width == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            buf_aux1.push(0xa);
                            buf_aux2.push((ci, width_seen, Normal));
                        }
                    } else if cl_width > max_width {
                        let len_nor = buf_aux2.len_nor();
                        buf_aux1.push_within(0xa, pi + len_nor, ci + len_nor);
                        buf_aux2.push((pi, width_seen - width_just_seen, Normal));
                    }
                }

                pi = ci;
            }

            if !is_last_cp {
                buf_aux1.push(bf_bytes[ci]);
                if bf_bytes[ci] == 0xa {
                    buf_aux2.push((ci, width_seen, Abnormal));
                }
            }
        }

        Ok(buf_aux1.len())
    }

    pub(crate) fn wrap_may_break_prepend(&mut self, prepend_str: &str) -> Result<usize> {
        use Aux2ElemKind::{Abnormal, Normal};

        let bf_bytes = self.before.as_bytes();
        let bf_len = bf_bytes.len();
        let max_width = self.max_width;

        if max_width < UnicodeWidthStr::width(prepend_str) {
            return Err(WrapError::InvalidWidth);
        }

        if bf_len == 0 {
            return Ok(0);
        }
        self.is_af_buf_suffice(
            bf_len + bf_len / max_width + prepend_str.len() * (bf_len / max_width),
        )?;

        let mut backing_buf_aux2 = (usize::MAX, usize::MAX, Aux2ElemKind::Normal);
        let mut buf_aux1 = Aux1Buf::new(self.after);
        let mut buf_aux2 = Aux2Buf::new(&mut backing_buf_aux2);
        let mut aux4 = 0usize;

        let prepend_bytes = prepend_str.as_bytes();
        let prepend_len = prepend_bytes.len();
        let mut width_seen = 0usize;
        let mut pi = 0;
        buf_aux1.push(bf_bytes[0]);

        let is_broken = |idx: usize| -> bool {
            if prepend_len > 0 {
                if idx == bf_bytes.len() {
                    bf_bytes[idx - 1] != 0x20 && bf_bytes[idx - 1] != 0xa
                } else {
                    bf_bytes[idx] != 0x20 && bf_bytes[idx - 1] != 0x20 && bf_bytes[idx - 1] != 0xa
                }
            } else {
                false
            }
        };

        for ci in 1..bf_len + 1 {
            let is_last_cp = ci == bf_len;

            if is_last_cp
                || (!is_last_cp && (bf_bytes[ci] < 0b10000000 || bf_bytes[ci] >= 0b11000000))
            {
                let width_just_seen = UnicodeWidthStr::width(&self.before[pi..ci]);
                width_seen += width_just_seen;

                if buf_aux2.len() == 0 {
                    if width_seen == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            if is_broken(ci) {
                                buf_aux1.push_many(prepend_bytes);
                                aux4 += prepend_len;
                            }
                            buf_aux1.push(0xa);
                            buf_aux2.push((ci, width_seen, Normal));
                        }
                    } else if width_seen > max_width {
                        if is_broken(pi) {
                            buf_aux1.ppush_within(prepend_bytes, 0xa, pi, ci);
                            aux4 += prepend_len;
                        } else {
                            buf_aux1.push_within(0xa, pi, ci);
                        }
                        buf_aux2.push((pi, width_seen - width_just_seen, Normal));
                    }
                } else {
                    let most_recent_aux2 = buf_aux2.last();
                    let (_, mark_aux2_i, _) = most_recent_aux2;
                    let cl_width = width_seen - mark_aux2_i;
                    if cl_width == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            if is_broken(ci) {
                                buf_aux1.push_many(prepend_bytes);
                                aux4 += prepend_len;
                            }
                            buf_aux1.push(0xa);
                            buf_aux2.push((ci, width_seen, Normal));
                        }
                    } else if cl_width > max_width {
                        let len_nor = buf_aux2.len_nor();

                        if is_broken(pi) {
                            buf_aux1.ppush_within(
                                prepend_bytes,
                                0xa,
                                pi + len_nor + aux4,
                                ci + len_nor + aux4,
                            );
                            aux4 += prepend_len;
                        } else {
                            buf_aux1.push_within(0xa, pi + len_nor + aux4, ci + len_nor + aux4);
                        }
                        buf_aux2.push((pi, width_seen - width_just_seen, Normal));
                    }
                }

                pi = ci;
            }

            if !is_last_cp {
                buf_aux1.push(bf_bytes[ci]);
                if bf_bytes[ci] == 0xa {
                    buf_aux2.push((ci, width_seen, Abnormal));
                }
            }
        }

        Ok(buf_aux1.len())
    }

    pub(crate) fn wrap_may_break_append(&mut self, append_str: &str) -> Result<usize> {
        use Aux2ElemKind::{Abnormal, Normal};

        let bf_bytes = self.before.as_bytes();
        let bf_len = bf_bytes.len();
        let max_width = self.max_width;
        let len_append = append_str.len();

        self.is_af_buf_suffice(bf_len + (bf_len / max_width) * (1 + len_append))?;

        let mut backing_buf_aux2 = (usize::MAX, usize::MAX, Aux2ElemKind::Normal);

        let mut buf_aux1 = Aux1Buf::new(self.after);
        let mut buf_aux2 = Aux2Buf::new(&mut backing_buf_aux2);
        let mut len_appended = 0;

        let bytes_append = append_str.as_bytes();
        let mut width_seen = 0usize;

        let mut pi = 0;
        buf_aux1.push(bf_bytes[0]);

        for ci in 1..bf_len + 1 {
            let is_last_cp = ci == bf_len;

            if is_last_cp
                || (!is_last_cp && (bf_bytes[ci] < 0b10000000 || bf_bytes[ci] >= 0b11000000))
            {
                let width_just_seen = UnicodeWidthStr::width(&self.before[pi..ci]);
                width_seen += width_just_seen;

                if buf_aux2.len() == 0 {
                    if width_seen == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            buf_aux1.push(0xa);
                            buf_aux1.push_many(bytes_append);
                            len_appended += len_append;
                            buf_aux2.push((ci, width_seen, Normal));
                        }
                    } else if width_seen > max_width {
                        buf_aux1.push_within(0xa, pi, ci);
                        buf_aux1.push_many_at(bytes_append, pi + 1);
                        len_appended += len_append;
                        buf_aux2.push((pi, width_seen - width_just_seen, Normal));
                    }
                } else {
                    let most_recent_aux2 = buf_aux2.last();
                    let (_, mark_aux2_i, _) = most_recent_aux2;
                    let cl_width = width_seen - mark_aux2_i;
                    if cl_width == max_width && !is_last_cp {
                        if bf_bytes[ci] != 0xa {
                            buf_aux1.push(0xa);
                            buf_aux1.push_many(bytes_append);
                            len_appended += len_append;
                            buf_aux2.push((ci, width_seen, Normal));
                        }
                    } else if cl_width > max_width {
                        let len_nor = buf_aux2.len_nor();
                        let len_aux = len_nor + len_appended;
                        buf_aux1.push_within(0xa, pi + len_aux, ci + len_aux);
                        buf_aux1.push_many_at(bytes_append, pi + len_aux + 1);
                        len_appended += len_append;
                        buf_aux2.push((pi, width_seen - width_just_seen, Normal));
                    }
                }

                pi = ci;
            }

            if !is_last_cp {
                buf_aux1.push(bf_bytes[ci]);
                if bf_bytes[ci] == 0xa {
                    buf_aux2.push((ci, width_seen, Abnormal));
                }
            }
        }

        Ok(buf_aux1.len())
    }

    //-
}
