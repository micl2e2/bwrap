#[cfg(feature = "use_std")]
mod stdonly {
    ///
    /// Wrap text with `N`-width limit, `N` defaults to 80 if omitted.
    ///
    /// Note that the style `WrapStyle::NoBrk` is in use, assuming no
    /// appending or prepending. This suits for the space-sensitive
    /// languages, such as English, French, German, etc.
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
}
