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
        let expected = "hel\nlo";

        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBrk(None, None))?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap_maybrk!(before, 3), expected);

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hello world";
        let expected = "hell\no wo\nrld";

        let mut wrapper = EasyWrapper::new(before, 4)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBrk(None, None))?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap_maybrk!(before, 4), expected);

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hello hello hello";
        let expected = "hell\no he\nllo \nhell\no";

        let mut wrapper = EasyWrapper::new(before, 4)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBrk(None, None))?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap_maybrk!(before, 4), expected);

        Ok(())
    }
    // -
}
