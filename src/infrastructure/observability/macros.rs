#[macro_export]
macro_rules! zone {
    ($name:literal) => {
        #[cfg(feature = "obs-frames")]
        profiling::scope!($name);
        #[cfg(all(feature = "obs-trace", not(feature = "obs-frames")))]
        let _obs_span = tracing::info_span!($name).entered();
    };
}

#[macro_export]
macro_rules! frame_mark {
    () => {
        #[cfg(feature = "obs-frames")]
        profiling::finish_frame!();
    };
}
