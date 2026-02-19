use std::ptr;

use ohos_media_sys as media;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
#[allow(deprecated)]
fn link_smoke() {
    unsafe {
        // native_media_core
        let _ = media::avformat::OH_AVFormat_Create();
        // native_media_core (avmemory)
        let _ = media::avmemory::OH_AVMemory_Create(0);
    }

    #[cfg(feature = "api-18")]
    touch_type::<media::avimage_generator_base::OH_AVImageGenerator_QueryOptions>();
    #[cfg(feature = "api-11")]
    touch_type::<media::avplayer_base::AVPlayerState>();
    #[cfg(feature = "api-18")]
    touch_type::<media::avrecorder_base::OH_AVRecorder>();
    touch_type::<media::avscreen_capture_base::OH_AVScreenCapture>();
    touch_type::<media::avscreen_capture_errors::OH_AVSCREEN_CAPTURE_ErrCode>();
    #[cfg(feature = "api-20")]
    touch_type::<media::avtranscoder_base::OH_AVTranscoder>();
    #[cfg(feature = "api-20")]
    touch_type::<media::lowpower_audio_sink_base::OH_LowPowerAudioSink>();
    #[cfg(feature = "api-20")]
    touch_type::<media::lowpower_video_sink_base::OH_LowPowerVideoSink>();
    #[cfg(feature = "api-18")]
    touch_type::<media::media_types::OH_Core_HdrType>();

    #[cfg(feature = "api-10")]
    unsafe {
        // native_media_codecbase
        let _ = media::avcapability::OH_AVCapability_GetName(ptr::null_mut());
        // native_media_adec
        let _ = media::avcodec_audiodecoder::OH_AudioDecoder_CreateByMime(ptr::null());
        // native_media_aenc
        let _ = media::avcodec_audioencoder::OH_AudioEncoder_CreateByMime(ptr::null());
        // native_media_avdemuxer
        let _ = media::avdemuxer::OH_AVDemuxer_CreateWithSource(ptr::null_mut());
        // native_media_avsource
        let _ = media::avsource::OH_AVSource_CreateWithFD(-1, 0, 0);
    }

    #[cfg(feature = "api-11")]
    unsafe {
        // native_media_acodec
        let _ = media::avcodec_audiocodec::OH_AudioCodec_CreateByMime(ptr::null(), false);
        // native_media_vdec
        let _ = media::avcodec_videodecoder::OH_VideoDecoder_CreateByMime(ptr::null());
        // native_media_venc
        let _ = media::avcodec_videoencoder::OH_VideoEncoder_RegisterCallback(
            ptr::null_mut(),
            std::mem::zeroed(),
            ptr::null_mut(),
        );
        // native_media_avdemuxer
        let _ =
            media::avdemuxer::OH_AVDemuxer_ReadSampleBuffer(ptr::null_mut(), 0, ptr::null_mut());
        // native_media_avmuxer
        let _ = media::avmuxer::OH_AVMuxer_Create(
            -1,
            media::avcodec_base::OH_AVOutputFormat::AV_OUTPUT_FORMAT_DEFAULT,
        );
        // native_media_core (avbuffer)
        let _ = media::avbuffer::OH_AVBuffer_Create(0);
        // avplayer
        let _ = media::avplayer::OH_AVPlayer_Create();
    }

    #[cfg(feature = "api-12")]
    unsafe {
        // native_media_codecbase (avcapability)
        let _ = media::avcapability::OH_AVCapability_IsFeatureSupported(
            ptr::null_mut(),
            std::mem::zeroed(),
        );
        // native_media_acodec
        let _ = media::avcodec_audiocodec::OH_AudioCodec_SetDecryptionConfig(
            ptr::null_mut(),
            ptr::null_mut(),
            false,
        );
        // native_media_vdec
        let _ = media::avcodec_videodecoder::OH_VideoDecoder_RenderOutputBufferAtTime(
            ptr::null_mut(),
            0,
            0,
        );
        // native_media_venc
        let _ = media::avcodec_videoencoder::OH_VideoEncoder_CreateByMime(ptr::null());
        // native_media_avdemuxer
        let _ = media::avdemuxer::OH_AVDemuxer_SetDemuxerMediaKeySystemInfoCallback(
            ptr::null_mut(),
            None,
        );
        // avplayer
        let _ = media::avplayer::OH_AVPlayer_SetMediaKeySystemInfoCallback(ptr::null_mut(), None);
        // native_media_avcencinfo
        let _ = media::cencinfo::OH_AVCencInfo_Create();
        // native_avscreen_capture
        let _ = media::avscreen_capture::OH_AVScreenCapture_SetStateCallback(
            ptr::null_mut(),
            None,
            ptr::null_mut(),
        );
        // native_media_avsource
        let _ = media::avsource::OH_AVSource_CreateWithDataSource(ptr::null_mut());
    }

    #[cfg(feature = "api-14")]
    unsafe {
        // native_media_avmuxer
        let _ = media::avmuxer::OH_AVMuxer_SetFormat(ptr::null_mut(), ptr::null_mut());
        // native_avscreen_capture
        let _ =
            media::avscreen_capture::OH_AVScreenCapture_SetMaxVideoFrameRate(ptr::null_mut(), 0);
    }

    #[cfg(feature = "api-15")]
    unsafe {
        // native_avscreen_capture
        let _ = media::avscreen_capture::OH_AVScreenCapture_ShowCursor(ptr::null_mut(), false);
    }

    #[cfg(feature = "api-18")]
    unsafe {
        // avimage_generator
        let _ = media::avimage_generator::OH_AVImageGenerator_Create();
        // avmetadata_extractor
        let _ = media::avmetadata_extractor::OH_AVMetadataExtractor_Create();
        // avrecorder
        let _ = media::avrecorder::OH_AVRecorder_Create();
        // native_media_core (avmemory)
        let _ = media::avmemory::OH_AVMemory_Create(0);
        // native_media_avsource
        let _ = media::avsource::OH_AVSource_GetCustomMetadataFormat(ptr::null_mut());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        // native_media_codecbase (avcapability)
        let _ = media::avcapability::OH_AVCapability_GetAudioSupportedSampleRateRanges(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        // native_media_acodec
        let _ = media::avcodec_audiocodec::OH_AudioCodec_QueryInputBuffer(
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        );
        // native_media_vdec
        let _ = media::avcodec_videodecoder::OH_VideoDecoder_QueryInputBuffer(
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        );
        // native_media_venc
        let _ = media::avcodec_videoencoder::OH_VideoEncoder_QueryInputBuffer(
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        );
        // native_media_core (avformat)
        let _ = media::avformat::OH_AVFormat_Create();
        // avplayer
        let _ = media::avplayer::OH_AVPlayer_SetPlaybackRate(ptr::null_mut(), 1.0);
        // avrecorder
        let _ = media::avrecorder::OH_AVRecorder_SetWillMuteWhenInterrupted(ptr::null_mut(), false);
        // avtranscoder
        let _ = media::avtranscoder::OH_AVTranscoder_Create();
        let _ = media::avtranscoder::OH_AVTranscoderConfig_Create();
        // native_avscreen_capture
        let _ = media::avscreen_capture::OH_AVScreenCapture_Create();
        // native_media_avsource
        let _ =
            media::avsource::OH_AVSource_CreateWithDataSourceExt(ptr::null_mut(), ptr::null_mut());
        // lowpower_avsink
        let _ = media::lowpower_audio_sink::OH_LowPowerAudioSink_CreateByMime(ptr::null());
        let _ = media::lowpower_video_sink::OH_LowPowerVideoSink_CreateByMime(ptr::null());
        let _ =
            media::lowpower_avsink_base::OH_AVSamplesBuffer_GetRemainedCapacity(ptr::null_mut());
        let _ = media::lowpower_avsink_base::OH_AVSamplesBuffer_AppendOneBuffer(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-21")]
    unsafe {
        // avplayer
        let _ = media::avplayer::OH_AVPlayer_SetLoudnessGain(ptr::null_mut(), 0.0);
        // lowpower_avsink
        let _ =
            media::lowpower_audio_sink::OH_LowPowerAudioSink_SetLoudnessGain(ptr::null_mut(), 0.0);
        let _ = media::lowpower_avsink_base::OH_LowPowerAVSink_GetCapability();
        let _ = media::lowpower_video_sink::OH_LowPowerVideoSink_GetLatestPts(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }
}
