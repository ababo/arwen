macro_rules! klog_debug {
    ($($arg:tt)*) => ({
        use klog;
        klog::log(klog::Level::Debug, format_args!($($arg)*));
    })
}

macro_rules! klog_info {
    ($($arg:tt)*) => ({
        use klog;
        klog::log(klog::Level::Info, format_args!($($arg)*));
    })
}

macro_rules! klog_warning {
    ($($arg:tt)*) => ({
        use klog;
        klog::log(klog::Level::Warning, format_args!($($arg)*));
    })
}

macro_rules! klog_error {
    ($($arg:tt)*) => ({
        use klog;
        klog::log(klog::Level::Error, format_args!($($arg)*));
    })
}

macro_rules! klog_fatal {
    ($($arg:tt)*) => ({
        use klog;
        klog::log(klog::Level::Fatal, format_args!($($arg)*));
    })
}

macro_rules! blocks_used {
    ($size:expr, $block_size:expr) => (($size + $block_size - 1) / $block_size)
}
