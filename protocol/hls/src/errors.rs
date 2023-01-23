use {
    failure::{Backtrace, Fail},
    rtmp::{
        amf0::errors::Amf0WriteError, cache::errors::MetadataError, session::errors::SessionError,
    },
    std::fmt,
    tokio::sync::broadcast::error::RecvError,
    xflv::errors::FlvDemuxerError,
    xmpegts::errors::MpegTsError,
};

#[derive(Debug)]
pub struct ServerError {
    pub value: ServerErrorValue,
}

#[derive(Debug, Fail)]
pub enum ServerErrorValue {
    #[fail(display = "server error")]
    Error,
}
#[derive(Debug)]
pub struct MediaError {
    pub value: MediaErrorValue,
}

#[derive(Debug, Fail)]
pub enum MediaErrorValue {
    #[fail(display = "server error")]
    Error,
    #[fail(display = "session error:{}\n", _0)]
    SessionError(#[cause] SessionError),
    #[fail(display = "amf write error:{}\n", _0)]
    Amf0WriteError(#[cause] Amf0WriteError),
    #[fail(display = "metadata error:{}\n", _0)]
    MetadataError(#[cause] MetadataError),
    #[fail(display = "flv demuxer error:{}\n", _0)]
    FlvDemuxerError(#[cause] FlvDemuxerError),
    #[fail(display = "mpegts error:{}\n", _0)]
    MpegTsError(#[cause] MpegTsError),
    #[fail(display = "write file error:{}\n", _0)]
    IOError(#[cause] std::io::Error),
}

impl From<SessionError> for MediaError {
    fn from(error: SessionError) -> Self {
        MediaError {
            value: MediaErrorValue::SessionError(error),
        }
    }
}

impl From<FlvDemuxerError> for MediaError {
    fn from(error: FlvDemuxerError) -> Self {
        MediaError {
            value: MediaErrorValue::FlvDemuxerError(error),
        }
    }
}

impl From<MpegTsError> for MediaError {
    fn from(error: MpegTsError) -> Self {
        MediaError {
            value: MediaErrorValue::MpegTsError(error),
        }
    }
}

impl From<Amf0WriteError> for MediaError {
    fn from(error: Amf0WriteError) -> Self {
        MediaError {
            value: MediaErrorValue::Amf0WriteError(error),
        }
    }
}

impl From<MetadataError> for MediaError {
    fn from(error: MetadataError) -> Self {
        MediaError {
            value: MediaErrorValue::MetadataError(error),
        }
    }
}

impl From<std::io::Error> for MediaError {
    fn from(error: std::io::Error) -> Self {
        MediaError {
            value: MediaErrorValue::IOError(error),
        }
    }
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl Fail for MediaError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.value.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.value.backtrace()
    }
}

pub struct HlsError {
    pub value: HlsErrorValue,
}

#[derive(Debug, Fail)]
pub enum HlsErrorValue {
    #[fail(display = "hls error")]
    Error,
    #[fail(display = "session error:{}\n", _0)]
    SessionError(#[cause] SessionError),
    #[fail(display = "amf write error:{}\n", _0)]
    Amf0WriteError(#[cause] Amf0WriteError),
    #[fail(display = "metadata error:{}\n", _0)]
    MetadataError(#[cause] MetadataError),
    #[fail(display = "flv demuxer error:{}\n", _0)]
    FlvDemuxerError(#[cause] FlvDemuxerError),
    #[fail(display = "media error:{}\n", _0)]
    MediaError(#[cause] MediaError),
    #[fail(display = "receive error:{}\n", _0)]
    RecvError(#[cause] RecvError),
}
impl From<RecvError> for HlsError {
    fn from(error: RecvError) -> Self {
        HlsError {
            value: HlsErrorValue::RecvError(error),
        }
    }
}

impl From<MediaError> for HlsError {
    fn from(error: MediaError) -> Self {
        HlsError {
            value: HlsErrorValue::MediaError(error),
        }
    }
}

impl From<SessionError> for HlsError {
    fn from(error: SessionError) -> Self {
        HlsError {
            value: HlsErrorValue::SessionError(error),
        }
    }
}

impl From<FlvDemuxerError> for HlsError {
    fn from(error: FlvDemuxerError) -> Self {
        HlsError {
            value: HlsErrorValue::FlvDemuxerError(error),
        }
    }
}

impl From<Amf0WriteError> for HlsError {
    fn from(error: Amf0WriteError) -> Self {
        HlsError {
            value: HlsErrorValue::Amf0WriteError(error),
        }
    }
}

impl From<MetadataError> for HlsError {
    fn from(error: MetadataError) -> Self {
        HlsError {
            value: HlsErrorValue::MetadataError(error),
        }
    }
}

impl fmt::Display for HlsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}
