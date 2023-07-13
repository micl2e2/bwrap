use bwrap::Result;
use bwrap::WrapStyle;
use bwrap::Wrapper;

mod ascii {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "hhhhh";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("-"), None))?;
        assert_eq!(&after[..len], "hhh-\nhh".as_bytes());

        assert_eq!(unicode_width::UnicodeWidthStr::width("⤶"), 1);
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;

        assert_eq!(&after[..len], "hhh⤶\nhh".as_bytes());

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh hhhhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "hhh⤶\nhh ⤶\nhhh⤶\nhh".as_bytes());

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hello hello hello";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "hell⤶\no he⤶\nllo ⤶\nhell⤶\no".as_bytes());

        Ok(())
    }
    // -
}

mod nonascii {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "ＨＨＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "ＨＨＨ⤶\nＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "ＨＨＨ⤶\nＨＨ Ｈ⤶\nＨＨＨ⤶\nＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ⤶\nＨＨ Ｈ⤶\nＨＨＨ⤶\nＨ ＨＨ⤶\nＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 315];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ⤶\nＨＨ Ｈ⤶\nＨＨＨ⤶\nＨ ＨＨ⤶\nＨＨＨ ⤶\nＨＨＨ⤶\nＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨhＨ ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 305];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ⤶\nＨＨ Ｈ⤶\nＨＨhＨ⤶\n ＨＨＨ⤶\nＨＨ Ｈ⤶\nＨＨＨ⤶\nＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "   ＨＨＨＨＨ   ＨＨＨＨＨ   ＨＨＨＨＨ   ";
        let mut after = [0u8; 285];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "   ＨＨ⤶\nＨＨＨ ⤶\n  ＨＨ⤶\nＨＨＨ ⤶\n  ＨＨ⤶\nＨＨＨ ⤶\n  ".as_bytes()
        );

        Ok(())
    }
}

mod nonascii_existnl {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _11() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ   \n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ   \n".as_bytes());

        Ok(())
    }

    #[test]
    fn _111() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ    \n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ   ⤶\n \n".as_bytes());

        Ok(())
    }

    #[test]
    fn _1111() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ           \n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ   ⤶\n       ⤶\n \n".as_bytes());

        Ok(())
    }

    #[test]
    fn _11111() -> Result<()> {
        let before = "     ＨＨＨ\nＨＨ           \n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "     Ｈ⤶\nＨＨ\nＨＨ   ⤶\n       ⤶\n \n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _111111() -> Result<()> {
        let before = "            ＨＨＨ\nＨＨ           \n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "       ⤶\n     Ｈ⤶\nＨＨ\nＨＨ   ⤶\n       ⤶\n \n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ Ｈ⤶\nＨＨＨ⤶\nＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ ＨＨ\nＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\nＨＨＨ⤶\nＨ ＨＨ\nＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨＨ ＨＨ\n\n\nＨＨＨ";
        let mut after = [0u8; 280];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨ⤶\nＨ ＨＨ\n\n\nＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        // similar to _4, but longer
        let before = "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨＨ ＨＨ\n\n\nＨＨＨ\n\n\n";
        let mut after = [0u8; 310];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨ⤶\nＨ ＨＨ\n\n\nＨＨＨ\n\n\n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        // similar to _5, but with one ascii
        let before = "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨh ＨＨ\n\n\nＨＨＨ\n\n\n";
        let mut after = [0u8; 300];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨh⤶\n ＨＨ\n\n\nＨＨＨ\n\n\n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 1, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(Some("⤶"), None))?;
        assert_eq!(
            &after[..len],
            "\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n⤶\nＨ\n".as_bytes()
        );

        Ok(())
    }

    //-
}
