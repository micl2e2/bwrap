use bwrap::ExistNlPref;
use bwrap::Result;
use bwrap::WrapStyle;
use bwrap::Wrapper;

mod ascii {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "hhhhh";
        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;

        assert_eq!(&after[..len], b"hhhhh");

        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh hhhhh";
        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;

        assert_eq!(&after[..len], b"hhhhh\n---hhhhh");
        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hh     hhh";
        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh \n---hhh");
        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "hhhhh hhhhh hhhhh";
        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\n---hhhhh\n---hhhhh");
        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "hhhhh   hhhhh   hhhhh";
        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\n---hhhhh\n---hhhhh");
        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "   hhhhh   hhhhh   hhhhh   ";
        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"  \n---hhhhh\n---hhhhh\n---hhhhh\n---");
        Ok(())
    }

    //-
}

mod ascii_existnl {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "hhh\nhh";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhh\nhh");

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhh\nhh");
        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "hhhhh\n hhhhh";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\n\n---hhhhh");

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hhhhh\nhhhhh");
        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "hh \n    hhh";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh \n   \n---hhh");

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh \nhhh");
        Ok(())
    }

    #[test]
    fn _33() -> Result<()> {
        let before = "hh \n  \n  hhh";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh \n  \n \n---hhh");

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh \n\nhhh");
        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "hh\nhhh hhh\nhh\n hhhhh";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"hh\nhhh\n---hhh\nhh\n\n---hhhhh");

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh\nhhh\n---hhh\nhh\nhhhhh");
        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "hh\n\n\nhhh   hhh\n\n\nhh\n\n\n   hhhhh";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"hh\n\n\nhhh\n---hhh\n\n\nhh\n\n\n  \n---hhhhh"
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"hh\n\n\nhhh\n---hhh\n\n\nhh\n\n\nhhhhh");
        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "\nhhhhh   hhhhh   hhhhh\n";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], b"\nhhhhh\n---hhhhh\n---hhhhh\n");

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], b"\nhhhhh\n---hhhhh\n---hhhhh\n");
        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "\nh\nh\nh\nh\nh   h\nh\nh\nh\nh   h\nh\nh\nh\nh\n";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh  \n---h\nh\nh\nh\nh  \n---h\nh\nh\nh\nh\n"
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh  \n---h\nh\nh\nh\nh  \n---h\nh\nh\nh\nh\n"
        );
        Ok(())
    }

    #[test]
    fn _77() -> Result<()> {
        let before = "\nh\nh\nh\nh\nh   h\nh\nh\nh\nh   h\nh\nh\nh\nh\n   ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh  \n---h\nh\nh\nh\nh  \n---h\nh\nh\nh\nh\n   "
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh  \n---h\nh\nh\nh\nh  \n---h\nh\nh\nh\nh\n"
        );
        Ok(())
    }

    #[test]
    fn _8() -> Result<()> {
        let before = "\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\n";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            b"\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\nh\n"
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
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

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ".as_bytes());

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ".as_bytes());
        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes());

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes());
        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "ＨＨＨＨＨ ＨＨＨhＨ ＨＨＨＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨhＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨhＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "ＨＨＨＨＨ   ＨＨＨhＨ   ＨＨＨＨＨ   ＨＨＨＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨhＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("---"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨＨＨ\n---ＨＨＨhＨ\n---ＨＨＨＨＨ\n---ＨＨＨＨＨ".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _8() -> Result<()> {
        let before = "   ＨＨＨＨＨ   ＨＨＨhＨ   ＨＨＨＨＨ   ＨＨＨＨＨ   ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("--"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "  \n--ＨＨＨＨＨ\n--ＨＨＨhＨ\n--ＨＨＨＨＨ\n--ＨＨＨＨＨ\n--".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("--"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "  \n--ＨＨＨＨＨ\n--ＨＨＨhＨ\n--ＨＨＨＨＨ\n--ＨＨＨＨＨ\n--".as_bytes()
        );
        Ok(())
    }
}

mod nonascii_existnl {
    use super::*;

    #[test]
    fn _1() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ".as_bytes());

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ".as_bytes());
        Ok(())
    }

    #[test]
    fn _2() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ ＨＨＨＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ\n⤷⤷⤷ＨＨＨＨＨ".as_bytes());

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ\n⤷⤷⤷ＨＨＨＨＨ".as_bytes());
        Ok(())
    }

    #[test]
    fn _3() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ".as_bytes());

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(&after[..len], "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ".as_bytes());
        Ok(())
    }

    #[test]
    fn _4() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ ＨＨＨ\nＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ\n⤷⤷⤷ＨＨＨ\nＨＨ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ\n⤷⤷⤷ＨＨＨ\nＨＨ".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _44() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈ\n ＨＨＨＨ ＨＨＨ\nＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\n\n⤷⤷⤷ＨＨＨＨ\n⤷⤷⤷ＨＨＨ\nＨＨ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ Ｈ\nＨＨＨＨ\n⤷⤷⤷ＨＨＨ\nＨＨ".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _5() -> Result<()> {
        let before = "ＨＨＨ\nＨＨ Ｈh\nＨＨＨ ＨhＨ\nＨＨ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ\n⤷⤷⤷Ｈh\nＨＨＨ\n⤷⤷⤷ＨhＨ\nＨＨ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ\n⤷⤷⤷Ｈh\nＨＨＨ\n⤷⤷⤷ＨhＨ\nＨＨ".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _55() -> Result<()> {
        let before = "ＨＨＨ\n ＨＨ Ｈh\nＨＨＨ ＨhＨ\n ＨＨ\n ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\n ＨＨ\n⤷⤷⤷Ｈh\nＨＨＨ\n⤷⤷⤷ＨhＨ\n ＨＨ\n ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "ＨＨＨ\nＨＨ\n⤷⤷⤷Ｈh\nＨＨＨ\n⤷⤷⤷ＨhＨ\nＨＨ\n".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _6() -> Result<()> {
        let before = "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 7, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _7() -> Result<()> {
        let before = "\nＨ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ Ｈ\nＨ\nＨ\nＨ\nＨ\n";

        let mut after = [0u8; 427];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ\n⤷⤷Ｈ\nＨ\nＨ\nＨ\nＨ\n⤷⤷Ｈ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );

        let mut after = [0u8; 427];
        let len = Wrapper::new(before, 2, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "\nＨ\nＨ\nＨ\nＨ\nＨ\n⤷⤷Ｈ\nＨ\nＨ\nＨ\nＨ\n⤷⤷Ｈ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _77() -> Result<()> {
        // similar to _7 but max_width is 3
        let before = " \nＨ\n Ｈ\nＨ\nＨ\nＨ Ｈ\nＨ\n Ｈ\nＨ\nＨ Ｈ\nＨ\n Ｈ\nＨ\nＨ\n ";

        let mut after = [0u8; 660];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            " \nＨ\n Ｈ\nＨ\nＨ\nＨ\n⤷⤷⤷Ｈ\nＨ\n Ｈ\nＨ\nＨ\n⤷⤷⤷Ｈ\nＨ\n Ｈ\nＨ\nＨ\n ".as_bytes()
        );

        let mut after = [0u8; 660];
        let len = Wrapper::new(before, 3, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            " \nＨ\nＨ\nＨ\nＨ\nＨ\n⤷⤷⤷Ｈ\nＨ\nＨ\nＨ\nＨ\n⤷⤷⤷Ｈ\nＨ\nＨ\nＨ\nＨ\n".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _8() -> Result<()> {
        let before = "     ＨＨＨＨ\nＨ     ＨＨＨhＨ     ＨＨＨＨＨ     ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \n⤷⤷⤷ＨＨＨＨ\nＨ  \n⤷⤷⤷ＨＨＨhＨ\n⤷⤷⤷ＨＨＨＨＨ\n⤷⤷⤷".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \n⤷⤷⤷ＨＨＨＨ\nＨ  \n⤷⤷⤷ＨＨＨhＨ\n⤷⤷⤷ＨＨＨＨＨ\n⤷⤷⤷".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _9() -> Result<()> {
        // note , we dont alter existing NL
        let before = "     ＨＨＨＨ\nＨ     ＨＨＨhＨ     ＨＨＨＨＨ \n   ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \n⤷⤷⤷ＨＨＨＨ\nＨ  \n⤷⤷⤷ＨＨＨhＨ\n⤷⤷⤷ＨＨＨＨＨ\n⤷⤷⤷\n   ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "    \n⤷⤷⤷ＨＨＨＨ\nＨ  \n⤷⤷⤷ＨＨＨhＨ\n⤷⤷⤷ＨＨＨＨＨ\n⤷⤷⤷\n".as_bytes()
        );
        Ok(())
    }

    #[test]
    fn _99() -> Result<()> {
        // note again, we dont alter existing NL
        let before = "  \n  ＨＨＨＨ\nＨ  \n  ＨＨＨhＨ  \n  ＨＨＨＨＨ \n   ";

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::KeepTrailSpc))?;
        assert_eq!(
            &after[..len],
            "  \n \n⤷⤷⤷ＨＨＨＨ\nＨ  \n \n⤷⤷⤷ＨＨＨhＨ\n⤷⤷⤷\n \n⤷⤷⤷ＨＨＨＨＨ\n⤷⤷⤷\n   ".as_bytes()
        );

        let mut after = [0u8; 1024];
        let len = Wrapper::new(before, 4, &mut after)?
            .wrap_use_style(WrapStyle::NoBrk(Some("⤷⤷⤷"), ExistNlPref::TrimTrailSpc))?;
        assert_eq!(
            &after[..len],
            "  \nＨＨＨＨ\nＨ  \nＨＨＨhＨ\n⤷⤷⤷\nＨＨＨＨＨ\n⤷⤷⤷\n".as_bytes()
        );
        Ok(())
    }
}
