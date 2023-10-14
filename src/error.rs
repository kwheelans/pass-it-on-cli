use thiserror::Error;

/// Errors returned by pass-it-on-command-line-client
#[derive(Error, Debug)]
pub enum Error {
    // ### Converting from other error types ###
    /// Pass-thru [`std::io::Error`].
    #[error("std::io Error: {0}")]
    StdIo(#[from] std::io::Error),

    /// Pass-thru
    #[error("Pass-it-on Error: {0}")]
    PassItOn(#[from] pass_it_on::Error),
}
