use opencv::imgcodecs;

pub enum ImReadFlags {
    AnyColor,
    AnyDepth,
    Color,
    Grayscale,
    IgnoreOrientation,
    LoadGDal,
    ReducedColor2,
    ReducedColor4,
    ReducedColor8,
    ReducedGrayscale2,
    ReducedGrayscale4,
    ReducedGrayscale8,
    Unchanged,
}
impl ImReadFlags {
    #[allow(dead_code)]
    pub fn from(flag: Self) -> i32 {
        let result = match flag {
            Self::AnyColor => imgcodecs::IMREAD_ANYCOLOR,
            Self::AnyDepth => imgcodecs::IMREAD_ANYDEPTH,
            Self::Color => imgcodecs::IMREAD_COLOR,
            Self::Grayscale => imgcodecs::IMREAD_GRAYSCALE,
            Self::IgnoreOrientation => imgcodecs::IMREAD_IGNORE_ORIENTATION,
            Self::LoadGDal => imgcodecs::IMREAD_LOAD_GDAL,
            Self::ReducedColor2 => imgcodecs::IMREAD_REDUCED_COLOR_2,
            Self::ReducedColor4 => imgcodecs::IMREAD_REDUCED_COLOR_4,
            Self::ReducedColor8 => imgcodecs::IMREAD_REDUCED_COLOR_8,
            Self::ReducedGrayscale2 => imgcodecs::IMREAD_REDUCED_GRAYSCALE_2,
            Self::ReducedGrayscale4 => imgcodecs::IMREAD_REDUCED_GRAYSCALE_4,
            Self::ReducedGrayscale8 => imgcodecs::IMREAD_REDUCED_GRAYSCALE_8,
            Self::Unchanged => imgcodecs::IMREAD_UNCHANGED,
        };

        return result;
    }
}
