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

#[cfg_attr(doc_cfg, doc(cfg(feature = "use_std")))]
#[cfg(feature = "use_std")]
mod stdonly {
    ///
    /// Wrap text in default style.
    ///
    /// The default style is `WrapStyle::NoBrk`, with no
    /// appending or prepending, and width limit defaults to 80 if second
    /// argument is omitted.
    ///
    /// [`Non-Break`]: crate::WrapStyle::NoBrk
    ///
    /// # Panics
    ///
    /// Panics if input string consists of invalid UTF8 bytes.
    #[macro_export]
    macro_rules! wrap {
        ($s:expr) => {{
            use bwrap::EasyWrapper;
            let mut wrapper = EasyWrapper::new($s, 80).expect("bwrap init");
            let w = wrapper.wrap().expect("bwrap wrap");
            String::from(w)
        }};
        ($s:expr, $mw:expr) => {{
            use bwrap::EasyWrapper;
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper.wrap().expect("bwrap wrap");
            String::from(w)
        }};
    }

    ///
    /// Wrap text in [`Non-Break`] style.
    ///
    /// [`Non-Break`]: crate::WrapStyle::NoBrk
    ///
    /// This macro has three forms:
    ///
    /// - `wrap_nobrk(A)`
    ///   - `A` is input string, assuming no appending.
    ///
    /// - `wrap_nobrk(A,B)`
    ///   - `A` is input string, `B` is width limit, assuming no appending.
    ///
    /// - `wrap_nobrk(A,B,C)`
    ///   - `A` is input string, `B` is width limit, `C` is appended string.
    ///
    /// # Panics
    ///
    /// Panics if input string consists of invalid UTF8 bytes.
    #[macro_export]
    macro_rules! wrap_nobrk {
        ($s:expr) => {{
            use bwrap::{EasyWrapper, ExistNlPref::KeepTrailSpc, WrapStyle::NoBrk};
            let mut wrapper = EasyWrapper::new($s, 80).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(bwrap::WrapStyle::NoBrk(None, KeepTrailSpc))
                .expect("bwrap wrap");
            String::from(w)
        }};

        ($s:expr, $mw:expr) => {{
            use bwrap::{EasyWrapper, ExistNlPref::KeepTrailSpc, WrapStyle::NoBrk};
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(bwrap::WrapStyle::NoBrk(None, KeepTrailSpc))
                .expect("bwrap wrap");
            String::from(w)
        }};

        ($s:expr, $mw:expr, $appd:expr) => {{
            use bwrap::{EasyWrapper, ExistNlPref::KeepTrailSpc, WrapStyle::NoBrk};
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(bwrap::WrapStyle::NoBrk(Some($appd), KeepTrailSpc))
                .expect("bwrap wrap");
            String::from(w)
        }};
    }

    ///
    /// Wrap text in [`May-Break`] style.
    ///
    /// [`May-Break`]: crate::WrapStyle::MayBrk
    ///
    /// This macro has three forms:
    ///
    /// - `wrap_maybrk(A)`
    ///   - `A` is input string, assuming 80-width limit and no prepending or appending.
    ///
    /// - `wrap_maybrk(A,B)`
    ///   - `A` is input string, `B` is width limit, assuming no prepending or appending.
    ///
    /// - `wrap_maybrk(A,B,C)`
    ///   - `A` is input string, `B` is width limit, `C` is prepended string, assuming no appending.
    ///
    /// - `wrap_maybrk(A,B,C,D)`
    ///   - `A` is input string, `B` is width limit, `C` is prepended string, `D` is appended string.
    ///
    /// # Panics
    ///
    /// Panics if input string consists of invalid UTF8 bytes.
    #[macro_export]
    macro_rules! wrap_maybrk {
        ($s:expr) => {{
            use bwrap::{EasyWrapper, WrapStyle};
            let mut wrapper = EasyWrapper::new($s, 80).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(WrapStyle::MayBrk(None, None))
                .expect("bwrap wrap");
            String::from(w)
        }};

        ($s:expr, $mw:expr) => {{
            use bwrap::{EasyWrapper, WrapStyle};
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(WrapStyle::MayBrk(None, None))
                .expect("bwrap wrap");
            String::from(w)
        }};

        ($s:expr, $mw:expr, $prep:expr) => {{
            use bwrap::{EasyWrapper, WrapStyle};
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(WrapStyle::MayBrk(Some($prep), None))
                .expect("bwrap wrap");
            String::from(w)
        }};

        ($s:expr, $mw:expr, $prep:expr, $appd:expr) => {{
            use bwrap::{EasyWrapper, WrapStyle};
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(WrapStyle::MayBrk(Some($prep), Some($appd)))
                .expect("bwrap wrap");
            String::from(w)
        }};
    }
}
