#[cfg(feature = "ext_fast_image_resize")]
mod feat {
    use crate::*;
    use fast_image_resize::FilterType;

    impl Ast for FilterType {
        fn ast() -> String {
            format!("*\"ext_fast_image_resize\"")
        }
    }

    impl Bytes for FilterType {
        fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
            use FilterType::*;

            let mut res = Self::default();
            if !buf.read_bool()? {
                return Ok(res);
            }

            let name = buf.read_string()?;

            Ok(match name.split_once("::").unwrap().1 {
                "Box" => Box,
                "CatmullRom" => CatmullRom,
                "Hamming" => Hamming,
                "Lanczos3" => Lanczos3,
                "Mitchell" => Mitchell,
                _ => return Err(MyErr::Todo),
            })
        }
    }
}
