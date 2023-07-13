// Copyright (C) 2023 Michael Lee <imichael2e2@proton.me/...@gmail.com>
//
// Licensed under the MIT License <LICENSE-MIT or
// https://opensource.org/license/mit> or the GNU General Public License,
// Version 3.0 or any later version <LICENSE-GPL or
// https://www.gnu.org/licenses/gpl-3.0.txt>, at your option.
//
// This file may not be copied, modified, or distributed except except in
// compliance with either of the licenses.
//

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![no_std]

#[cfg(any(feature = "use_std"))]
extern crate std;

mod error;

pub use error::Result;
pub use error::WrapError;

mod auxbuf;
#[cfg_attr(doc_cfg, doc(cfg(feature = "use_std")))]
#[cfg(feature = "use_std")]
mod easy_wrapper;
mod wrapper;

pub use wrapper::ExistNlPref;
pub use wrapper::WrapStyle;
pub use wrapper::Wrapper;

#[cfg_attr(doc_cfg, doc(cfg(feature = "use_std")))]
#[cfg(feature = "use_std")]
pub use easy_wrapper::EasyWrapper;
