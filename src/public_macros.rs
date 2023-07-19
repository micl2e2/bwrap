#[cfg(feature = "use_std")]
mod stdonly {
    use crate::EasyWrapper;
    use crate::WrapStyle;

    #[macro_export]
    macro_rules! wrap {
        ($s:expr) => {{
            let mut wrapper = EasyWrapper::new($s, 80).expect("bwrap init");
            let w = wrapper.wrap().expect("bwrap wrap");
            String::from(w)
        }};
        ($s:expr, $mw:expr) => {{
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper.wrap().expect("bwrap wrap");
            String::from(w)
        }};
    }
}
