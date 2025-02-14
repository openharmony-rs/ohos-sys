//! Provides APIs of Playback capability for Media Source.
//!
//! You can refer to the following guides for usage of the provided APIs
//!
//! - [Using AVPlayer to play Audio](https://docs.openharmony.cn/pages/v5.0/en/application-dev/media/media/using-ndk-avplayer-for-playback.md)
//! - [Using AVPlayer to play video](https://docs.openharmony.cn/pages/v5.0/en/application-dev/media/media/using-ndk-avplayer-for-video-playback.md)
//!
//! Reference: <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-media-kit/_a_v_player.md>
mod avplayer_ffi;
pub use avplayer_ffi::*;

#[link(name = "avplayer")]
extern "C" {}
