use bwrap::Result;
use bwrap::WrapStyle;

// all intput and output are identical to corresponding ones in nostd tests

#[cfg(feature = "use_std")]
mod ascii {
    use super::*;
    use bwrap::EasyWrapper;

    #[test]
    fn _1() -> Result<()> {
        let before = "hhhhh";

        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBrk(Some("-"), None))?;
        assert_eq!(&after, "hhh-\nhh");

        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after, "hhh⤶\nhh");

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after, "hhh⤶\nhh ⤶\nhhh⤶\nhh");

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hello hello hello";
        let mut wrapper = EasyWrapper::new(before, 4)?;
        let after = wrapper.wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after, "hell⤶\no he⤶\nllo ⤶\nhell⤶\no");

        Ok(())
    }
    // --
}
