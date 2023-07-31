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

//!
//! A fast, lightweight, embedded systems-friendly library for wrapping text.
//!
//! While Bwrap offers great flexibility in wrapping text, neither resource
//! consumption nor performance compromises:
//!
//! 1. No heap allocation happens by default.
//! 2. The time/space complexity is *O(n)* by default, or *O(n(p+a))* if there
//! is appending/prepending. (n, p, a is the number of input/prepended/appended
//! bytes respectively)
//!
//! For the sake of readability, we (**b**)etter **wrap** our text.
//!
//! # Features
//!
//! `use_std`: Use standard library for automatic memory management. (disable by default)
//!
//! # Examples
//!
//! _Note: These examples require `use_std` feature_.
//!
//! ---
//!
//! Wrap a long line of text:
//!
//! ```
//! let original = "
//! one two three one two three one two three
//! ";
//! let wrapped = bwrap::wrap!(original, 13);
//!
//! let expected = "
//! one two three
//! one two three
//! one two three
//! ";
//! assert_eq!(wrapped, expected);
//! ```
//!
//! Format CLI error messages:
//!
//! ```
//! let original = "
//! error: TLS handshake erorr.
//!   tip: Probably because of instable network connection, please try these solutions: Retry the connection; Use another different network; Use another version of TLS; Reboot your system; Reinstall your system; Give up.
//! ";
//! let wrapped = bwrap::wrap_nobrk!(original, 38, "       ");
//!
//! let expected = "
//! error: TLS handshake erorr.
//!   tip: Probably because of instable
//!        network connection, please try these
//!        solutions: Retry the connection; Use
//!        another different network; Use another
//!        version of TLS; Reboot your system;
//!        Reinstall your system; Give up.
//! ";
//! assert_eq!(wrapped, expected);
//! ```
//!
//! Format a bullet list:
//! ```
//! let original = "
//! Here is our schedule:
//! - Do A, and do B, and do C, and do D, and do E, and do F
//! - Do G, and do H, and do I, and do J, and do K, and do L
//! ";
//! let wrapped = bwrap::wrap_nobrk!(original, 35, "  ");
//!
//! let expected = "
//! Here is our schedule:
//! - Do A, and do B, and do C, and do
//!   D, and do E, and do F
//! - Do G, and do H, and do I, and do
//!   J, and do K, and do L
//! ";
//! assert_eq!(wrapped, expected);
//! ```
//!
//! Trailing notation:
//!
//! ```
//! let original = "
//! VGhpcyBpcyBhIHNlY3JldCBtZXNzYWdlLCBwbGVhc2UgZGVsZXRlIGFmdGVyIHJlYWQK
//! ";
//! let wrapped = bwrap::wrap_maybrk!(original, 10, " |");
//!
//! let expected = "
//! VGhpcyBpcy |
//! BhIHNlY3Jl |
//! dCBtZXNzYW |
//! dlLCBwbGVh |
//! c2UgZGVsZX |
//! RlIGFmdGVy |
//! IHJlYWQK
//! ";
//! assert_eq!(wrapped, expected);
//! ```
//!
//! More exmaples/benchmark can be found in [repository](https://github.com/micl2e2/bwrap).
//!
//!

#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
// #![feature(doc_auto_cfg)] // DEBUG ONLY

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

#[macro_use]
mod public_macros;
