use bwrap::Result;
use bwrap::WrapStyle;

// all intput and output are identical to corresponding ones in nostd tests

#[cfg(feature = "use_std")]
mod ascii {
    use super::*;
    use bwrap::EasyWrapper;

    #[test]
    fn _1() -> Result<()> {
        let before = "hello";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBreak)?;
        assert_eq!(&after, "hel\nlo");

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hello world";
        let mut wrapper = EasyWrapper::new(before, 4)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBreak)?;
        assert_eq!(&after, "hell\no wo\nrld");

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hello hello hello";
        let mut wrapper = EasyWrapper::new(before, 4)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBreak)?;
        assert_eq!(&after, "hell\no he\nllo \nhell\no");

        Ok(())
    }
    // -
}
