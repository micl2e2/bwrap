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

///
/// The internal errors.
#[derive(Debug)]
pub enum WrapError {
    InsufficentBufferSize(usize, usize, usize),
    InvalidWidth,
}

pub type Result<T> = core::result::Result<T, WrapError>;
