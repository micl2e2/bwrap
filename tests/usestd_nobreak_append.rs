use bwrap::ExistNlPref;
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
        let after =
            wrapper.wrap_use_style(WrapStyle::NoBreakAppend("---", ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after, "hhhhh");

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after =
            wrapper.wrap_use_style(WrapStyle::NoBreakAppend("---", ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after, "hhhhh\n---hhhhh");

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hh     hhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after =
            wrapper.wrap_use_style(WrapStyle::NoBreakAppend("---", ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after, "hh \n---hhh");

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "hhhhh hhhhh hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after =
            wrapper.wrap_use_style(WrapStyle::NoBreakAppend("---", ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after, "hhhhh\n---hhhhh\n---hhhhh");

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "hhhhh   hhhhh   hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after =
            wrapper.wrap_use_style(WrapStyle::NoBreakAppend("---", ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after, "hhhhh\n---hhhhh\n---hhhhh");

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "   hhhhh   hhhhh   hhhhh   ";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after =
            wrapper.wrap_use_style(WrapStyle::NoBreakAppend("---", ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after, "  \n---hhhhh\n---hhhhh\n---hhhhh\n---");

        Ok(())
    }

    //-
}
