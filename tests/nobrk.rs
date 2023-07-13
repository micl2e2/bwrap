use bwrap::ExistNlPref;
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
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh");

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh hhhhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hh     hhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh \nhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh \nhhh");

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "hhhhh hhhhh hhhhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh\nhhhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "hhhhh   hhhhh   hhhhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh\nhhhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "   hhhhh   hhhhh   hhhhh   ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"  \nhhhhh\nhhhhh\nhhhhh\n");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"  \nhhhhh\nhhhhh\nhhhhh\n");

        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 80, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS\nFOR A PARTICULAR PURPOSE".as_bytes());

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 80, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS\nFOR A PARTICULAR PURPOSE".as_bytes());

        Ok(())
    }

    //-
}

mod ascii_existnl {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "hhh\nhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhh\nhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhh\nhh");

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh\n hhhhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\n\nhhhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hh \n    hhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh \n   \nhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh \nhhh");

        Ok(())
    }

    #[test]
    fn _33() -> Result<()> {
        let before = "hh \n  \n  hhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh \n  \n \nhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh \n\nhhh");

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "hh\nhhh hhh\nhh\n hhhhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh\nhhh\nhhh\nhh\n\nhhhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh\nhhh\nhhh\nhh\nhhhhh");

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "hh\n\n\nhhh   hhh\n\n\nhh\n\n\n   hhhhh";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh\n\n\nhhh\nhhh\n\n\nhh\n\n\n  \nhhhhh");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh\n\n\nhhh\nhhh\n\n\nhh\n\n\nhhhhh");

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "\nhhhhh   hhhhh   hhhhh\n";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"\nhhhhh\nhhhhh\nhhhhh\n");

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"\nhhhhh\nhhhhh\nhhhhh\n");

        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "\nh\nh\nh\nh\nh   h\nh\nh\nh\nh   h\nh\nh\nh\nh\n";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh  \nh\nh\nh\nh\nh  \nh\nh\nh\nh\nh\n"
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh  \nh\nh\nh\nh\nh  \nh\nh\nh\nh\nh\n"
        );

        Ok(())
    }

    #[test]
    fn _8() -> Result<()> {
        let before = "\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\n";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\n"
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\n"
        );

        Ok(())
    }

    //-
}

mod nonascii {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "ＨＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ".as_bytes());

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes());

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨＨＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨＨＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨhＨ ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨhＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨhＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "ＨＨＨＨＨ   ＨＨＨhＨ   ＨＨＨＨＨ   ＨＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨhＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\nＨＨＨhＨ\nＨＨＨＨＨ\nＨＨＨＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _8() -> Result<()> {
        let before = "   ＨＨＨＨＨ   ＨＨＨhＨ   ＨＨＨＨＨ   ＨＨＨＨＨ   ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "  \nＨＨＨＨＨ\nＨＨＨhＨ\nＨＨＨＨＨ\nＨＨＨＨＨ\n".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "  \nＨＨＨＨＨ\nＨＨＨhＨ\nＨＨＨＨＨ\nＨＨＨＨＨ\n".as_bytes()
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
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ".as_bytes());

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ\nＨＨＨＨＨ".as_bytes());

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ\nＨＨＨＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ".as_bytes());

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ".as_bytes());

        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ ＨＨＨ\nＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ\nＨＨＨ\nＨＨ".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ\nＨＨＨ\nＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈh\nＨＨＨ ＨhＨ\nＨＨ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ\nＨh\nＨＨＨ\nＨhＨ\nＨＨ".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ\nＨh\nＨＨＨ\nＨhＨ\nＨＨ".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _8() -> Result<()> {
        let before = "     ＨＨＨＨ\nＨ     ＨＨＨhＨ     ＨＨＨＨＨ     ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \nＨＨＨＨ\nＨ  \nＨＨＨhＨ\nＨＨＨＨＨ\n".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \nＨＨＨＨ\nＨ  \nＨＨＨhＨ\nＨＨＨＨＨ\n".as_bytes()
        );

        Ok(())
    }

    #[test]
    fn _9() -> Result<()> {
        let before = "     ＨＨＨＨ\nＨ     ＨＨＨhＨ     ＨＨＨＨＨ \n   ";

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \nＨＨＨＨ\nＨ  \nＨＨＨhＨ\nＨＨＨＨＨ\n\n   ".as_bytes()
        );

        let mut after = [0u8; 256];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(None, ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \nＨＨＨＨ\nＨ  \nＨＨＨhＨ\nＨＨＨＨＨ\n\n".as_bytes()
        );

        Ok(())
    }
}
