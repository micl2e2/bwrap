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

// ByteSlcBuf //

pub(crate) struct ByteSlcBuf<'a> {
    nexi: usize,
    inner: &'a mut [u8],
}

impl<'a> ByteSlcBuf<'a> {
    pub(crate) fn new(slc: &'a mut [u8]) -> Self {
        ByteSlcBuf {
            nexi: 0,
            inner: slc,
        }
    }

    pub(crate) fn push(&mut self, byte: u8) {
        if self.nexi >= self.inner.len() {
            panic!("buffer overflow {} {}", self.nexi, self.inner.len());
        }
        self.inner[self.nexi] = byte;
        self.nexi += 1;
    }

    pub(crate) fn push_many(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.push(*byte);
        }
    }

    pub(crate) fn push_many_at(&mut self, bytes: &[u8], begi: usize) {
        let many_len = bytes.len();
        self.inner.copy_within(begi..self.nexi, begi + many_len);
        self.nexi += many_len;
        for i in 0..many_len {
            self.inner[begi + i] = bytes[i];
        }
    }

    pub(crate) fn push_within(&mut self, byte: u8, begi: usize, endi: usize) {
        self.inner.copy_within(begi..endi, begi + 1);
        self.inner[begi] = byte;
        self.nexi += 1;
    }

    pub(crate) fn ppush_within(&mut self, prep_bytes: &[u8], byte: u8, begi: usize, endi: usize) {
        let prep_len = prep_bytes.len();
        self.inner.copy_within(begi..endi, begi + prep_len + 1);
        for i in begi..begi + prep_len {
            self.inner[i] = prep_bytes[i - begi];
        }
        self.inner[begi + prep_len] = byte;
        self.nexi += 1 + prep_len;
    }

    pub(crate) fn replace(&mut self, idx: usize, newbyte: u8) {
        if idx >= self.nexi {
            panic!("index {} is out of bound {}", idx, self.nexi);
        }
        self.inner[idx] = newbyte;
    }

    pub(crate) fn len(&self) -> usize {
        self.nexi
    }

    pub(crate) fn last(&self) -> u8 {
        self.inner[self.nexi - 1]
    }

    pub(crate) fn last_many_equal(&self, many_len: usize, first: u8, trailing: &[u8]) -> bool {
        if self.inner[self.nexi - many_len] != first {
            return false;
        }
        if &self.inner[self.nexi - many_len + 1..self.nexi] != trailing {
            return false;
        }

        true
    }
}

// HypoNlBuf //

#[derive(Debug, Copy, Clone)]
pub(crate) enum HypoNlKind {
    Existing,
    NewOne,
}

pub(crate) struct HypoNlBuf<'a> {
    nexi: usize,
    inner: &'a mut (usize, usize, HypoNlKind),
    n_exis: usize,
}

impl<'a> HypoNlBuf<'a> {
    pub(crate) fn new(slc: &'a mut (usize, usize, HypoNlKind)) -> Self {
        HypoNlBuf {
            nexi: 0,
            inner: slc,
            n_exis: 0,
        }
    }

    pub(crate) fn push(&mut self, arg: (usize, usize, HypoNlKind)) {
        *self.inner = (arg.0, arg.1, arg.2);
        self.nexi += 1;
        if let HypoNlKind::Existing = arg.2 {
            self.n_exis += 1;
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.nexi
    }

    pub(crate) fn len_newones(&self) -> usize {
        self.nexi - self.n_exis
    }

    pub(crate) fn last(&self) -> &(usize, usize, HypoNlKind) {
        &self.inner
    }
}

// ExisSpcBuf //

pub(crate) struct ExisSpcBuf<'a> {
    nexi: usize,
    inner: &'a mut (usize, usize),
}

impl<'a> ExisSpcBuf<'a> {
    pub(crate) fn new(slc: &'a mut (usize, usize)) -> Self {
        ExisSpcBuf {
            nexi: 0,
            inner: slc,
        }
    }

    pub(crate) fn push(&mut self, arg: (usize, usize)) {
        *self.inner = (arg.0, arg.1);
        self.nexi += 1;
    }

    pub(crate) fn len(&self) -> usize {
        self.nexi
    }

    pub(crate) fn last(&self) -> &(usize, usize) {
        &self.inner
    }
}
