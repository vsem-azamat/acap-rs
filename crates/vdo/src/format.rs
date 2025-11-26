//! Video format types.

use vdo_sys::VdoFormat as RawVdoFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Format {
    H264,
    H265,
    Jpeg,
    Yuv,
    Bayer,
    Ivs,
    Raw,
    Rgba,
    Rgb,
    PlanarRgb,
    Av1,
}

impl Format {
    pub(crate) fn to_raw(self) -> RawVdoFormat {
        match self {
            Self::H264 => RawVdoFormat::VDO_FORMAT_H264,
            Self::H265 => RawVdoFormat::VDO_FORMAT_H265,
            Self::Jpeg => RawVdoFormat::VDO_FORMAT_JPEG,
            Self::Yuv => RawVdoFormat::VDO_FORMAT_YUV,
            Self::Bayer => RawVdoFormat::VDO_FORMAT_BAYER,
            Self::Ivs => RawVdoFormat::VDO_FORMAT_IVS,
            Self::Raw => RawVdoFormat::VDO_FORMAT_RAW,
            Self::Rgba => RawVdoFormat::VDO_FORMAT_RGBA,
            Self::Rgb => RawVdoFormat::VDO_FORMAT_RGB,
            Self::PlanarRgb => RawVdoFormat::VDO_FORMAT_PLANAR_RGB,
            Self::Av1 => RawVdoFormat::VDO_FORMAT_AV1,
        }
    }

    pub(crate) fn as_i32(self) -> i32 {
        self.to_raw().0
    }

    #[allow(dead_code)]
    pub(crate) fn from_raw(raw: RawVdoFormat) -> Option<Self> {
        match raw {
            RawVdoFormat::VDO_FORMAT_H264 => Some(Self::H264),
            RawVdoFormat::VDO_FORMAT_H265 => Some(Self::H265),
            RawVdoFormat::VDO_FORMAT_JPEG => Some(Self::Jpeg),
            RawVdoFormat::VDO_FORMAT_YUV => Some(Self::Yuv),
            RawVdoFormat::VDO_FORMAT_BAYER => Some(Self::Bayer),
            RawVdoFormat::VDO_FORMAT_IVS => Some(Self::Ivs),
            RawVdoFormat::VDO_FORMAT_RAW => Some(Self::Raw),
            RawVdoFormat::VDO_FORMAT_RGBA => Some(Self::Rgba),
            RawVdoFormat::VDO_FORMAT_RGB => Some(Self::Rgb),
            RawVdoFormat::VDO_FORMAT_PLANAR_RGB => Some(Self::PlanarRgb),
            RawVdoFormat::VDO_FORMAT_AV1 => Some(Self::Av1),
            _ => None,
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::H264 => "H.264",
            Self::H265 => "H.265",
            Self::Jpeg => "JPEG",
            Self::Yuv => "YUV",
            Self::Bayer => "Bayer",
            Self::Ivs => "IVS",
            Self::Raw => "Raw",
            Self::Rgba => "RGBA",
            Self::Rgb => "RGB",
            Self::PlanarRgb => "Planar RGB",
            Self::Av1 => "AV1",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FrameType {
    None,
    H264Sps,
    H264Pps,
    H264Sei,
    H264Idr,
    H264I,
    H264P,
    H264B,
    H265Sps,
    H265Pps,
    H265Vps,
    H265Sei,
    H265Idr,
    H265I,
    H265P,
    H265B,
    Jpeg,
    Yuv,
    Raw,
    Rgba,
    Rgb,
    PlanarRgb,
    Av1Key,
    Av1Inter,
    Av1Bidi,
}

impl FrameType {
    pub fn is_keyframe(&self) -> bool {
        matches!(
            self,
            Self::H264Idr | Self::H264I | Self::H265Idr | Self::H265I | Self::Av1Key | Self::Jpeg
        )
    }

    pub(crate) fn from_raw(raw: vdo_sys::VdoFrameType) -> Option<Self> {
        match raw {
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_NONE => Some(Self::None),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H264_SPS => Some(Self::H264Sps),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H264_PPS => Some(Self::H264Pps),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H264_SEI => Some(Self::H264Sei),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H264_IDR => Some(Self::H264Idr),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H264_I => Some(Self::H264I),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H264_P => Some(Self::H264P),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H264_B => Some(Self::H264B),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_SPS => Some(Self::H265Sps),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_PPS => Some(Self::H265Pps),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_VPS => Some(Self::H265Vps),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_SEI => Some(Self::H265Sei),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_IDR => Some(Self::H265Idr),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_I => Some(Self::H265I),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_P => Some(Self::H265P),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_H265_B => Some(Self::H265B),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_JPEG => Some(Self::Jpeg),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_YUV => Some(Self::Yuv),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_RAW => Some(Self::Raw),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_RGBA => Some(Self::Rgba),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_RGB => Some(Self::Rgb),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_PLANAR_RGB => Some(Self::PlanarRgb),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_AV1_KEY => Some(Self::Av1Key),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_AV1_INTER => Some(Self::Av1Inter),
            vdo_sys::VdoFrameType::VDO_FRAME_TYPE_AV1_BIDI => Some(Self::Av1Bidi),
            _ => None,
        }
    }
}
