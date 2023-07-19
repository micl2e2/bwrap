// all intput and output are identical to corresponding ones in nostd tests

#[cfg(feature = "use_std")]
mod ascii {
    use super::*;
    use bwrap::Result;

    #[test]
    fn _1() -> Result<()> {
        let before = "hhhhh";
        let expected = "hhhhh";

        let mut wrapper = bwrap::EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap!(before, 3), expected);

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh hhhhh";
        let expected = "hhhhh\nhhhhh";

        let mut wrapper = bwrap::EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap!(before, 3), expected);

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hh     hhh";
        let expected = "hh \nhhh";

        let mut wrapper = bwrap::EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap!(before, 3), expected);

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "hhhhh hhhhh hhhhh";
        let expected = "hhhhh\nhhhhh\nhhhhh";

        let mut wrapper = bwrap::EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap!(before, 3), expected);

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "hhhhh   hhhhh   hhhhh";
        let expected = "hhhhh\nhhhhh\nhhhhh";

        let mut wrapper = bwrap::EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap!(before, 3), expected);

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "   hhhhh   hhhhh   hhhhh   ";
        let expected = "  \nhhhhh\nhhhhh\nhhhhh\n";

        let mut wrapper = bwrap::EasyWrapper::new(before, 3)?;
        let after = wrapper.wrap()?;

        assert_eq!(&after, expected);
        assert_eq!(bwrap::wrap!(before, 3), expected);

        Ok(())
    }

    //-
}
