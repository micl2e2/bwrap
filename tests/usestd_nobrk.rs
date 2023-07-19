use bwrap::Result;

// all intput and output are identical to corresponding ones in nostd tests

#[cfg(feature = "use_std")]
mod ascii {
    use super::*;
    use bwrap::EasyWrapper;

    #[test]
    fn _1() -> Result<()> {
        let before = "hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;
        assert_eq!(&after, "hhhhh");
        assert_eq!(bwrap::wrap!(before, 3), "hhhhh");

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, "hhhhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hh     hhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, "hh \nhhh");

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "hhhhh hhhhh hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, "hhhhh\nhhhhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "hhhhh   hhhhh   hhhhh";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, "hhhhh\nhhhhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "   hhhhh   hhhhh   hhhhh   ";
        let mut wrapper = EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, "  \nhhhhh\nhhhhh\nhhhhh\n");

        Ok(())
    }

    //-
}
