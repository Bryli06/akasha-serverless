use cfg_if::cfg_if;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub enum Error {
    EnvironmentVariableNotFound(String),
    HeaderNotFound(String),
    JsonFailed(serde_json::Error),
    PayloadError(String),
    VerificationFailed(VerificationError),
    InteractionFailed(InteractionError)
}

pub enum InteractionError {
    CommandNotFound(String),
    CloudflareError(worker::Error),
    DiscordError(String),
    Error(),
}

pub enum VerificationError {
    ParseError()
}
