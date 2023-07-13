use bwrap::Result;
use bwrap::WrapStyle;
use bwrap::Wrapper;

mod ascii {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "hello";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], b"hel\n~~~lo");

        Ok(())
    }
    #[test]
    fn _2() -> Result<()> {
        let before = "hello world";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], b"hell\n~~~o wo\n~~~rld");

        Ok(())
    }
    #[test]
    fn _3() -> Result<()> {
        let before = "hello hello hello";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], b"hell\n~~~o he\n~~~llo \n~~~hell\n~~~o");

        Ok(())
    }
    // -
}

mod ascii_existnl {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "hel\nlo";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "hel\nlo".as_bytes());

        Ok(())
    }
    #[test]
    fn _2() -> Result<()> {
        let before = "hel\nlo \nworld";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "hel\nlo \nwor\n~~~ld".as_bytes());

        Ok(())
    }
    #[test]
    fn _3() -> Result<()> {
        let before = "hel\nlo \nwor\nld";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "hel\nlo \nwor\nld".as_bytes());

        Ok(())
    }
    #[test]
    fn _4() -> Result<()> {
        let before = "\nhell\no";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "\nhel\n~~~l\no".as_bytes());

        Ok(())
    }
    #[test]
    fn _5() -> Result<()> {
        let before = "\nhhhhh\nhhhhh\n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "\nhhh\n~~~hh\nhhh\n~~~hh\n".as_bytes());

        Ok(())
    }
    #[test]
    fn _6() -> Result<()> {
        let before = "\nh\nh\nh\nh\nh\nh\nhhhh\n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "\nh\nh\nh\nh\nh\nh\nhhh\n~~~h\n".as_bytes());

        Ok(())
    }
    #[test]
    fn _7() -> Result<()> {
        let before = "\n\n\n\n\nhhhhh\n\n\n\n\nhhhhh\n\n\n\n\n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "\n\n\n\n\nhhh\n~~~hh\n\n\n\n\nhhh\n~~~hh\n\n\n\n\n".as_bytes()
        );

        Ok(())
    }
    #[test]
    fn _8() -> Result<()> {
        let before = "\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 1, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\n".as_bytes());

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
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "ＨＨＨ\n~~~ＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\n~~~ＨＨ Ｈ\n~~~ＨＨＨ\n~~~Ｈ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\n~~~ＨＨ Ｈ\n~~~ＨＨＨ\n~~~Ｈ ＨＨ\n~~~ＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 315];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\n~~~ＨＨ Ｈ\n~~~ＨＨＨ\n~~~Ｈ ＨＨ\n~~~ＨＨＨ \n~~~ＨＨＨ\n~~~ＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨhＨ ＨＨＨＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 305];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\n~~~ＨＨ Ｈ\n~~~ＨＨhＨ\n~~~ ＨＨＨ\n~~~ＨＨ Ｈ\n~~~ＨＨＨ\n~~~Ｈ".as_bytes()
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
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ ＨＨＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\n~~~ＨＨＨ\n~~~Ｈ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ ＨＨ\nＨＨＨ";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\nＨＨＨ\n~~~Ｈ ＨＨ\nＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨＨ ＨＨ\n\n\nＨＨＨ";
        let mut after = [0u8; 280];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨ\n~~~Ｈ ＨＨ\n\n\nＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨＨ ＨＨ\n\n\nＨＨＨ\n\n\n";
        let mut after = [0u8; 310];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨ\n~~~Ｈ ＨＨ\n\n\nＨＨＨ\n\n\n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        // similar to _5, but with one ascii
        let before = "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨh ＨＨ\n\n\nＨＨＨ\n\n\n";
        let mut after = [0u8; 300];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "\n\n\nＨＨＨ\n\n\nＨＨ Ｈ\n\n\nＨＨＨh\n~~~ ＨＨ\n\n\nＨＨＨ\n\n\n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        // note, compared to ascii_existnl::_8, similar input but
        //       very different output. As for this one, max_width
        //       is inside a unicode code point, hence NL will
        //       be inserted anyway.
        //
        let before = "\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\n";
        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 1, &mut after)?
            .wrap_use_style(WrapStyle::MayBrk(None, Some("~~~")))?;
        assert_eq!(
            &after[..len],
            "\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n\n~~~Ｈ\n".as_bytes()
        );
        Ok(())
    }
    //-
}
