use std::fmt;

/// The error type reported by the different transports when something is wrong.
///
/// Certain transports will only produce certain variants of this error.
///
/// Unless the variant is `MissingBytes`, the connection should not continue.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Not enough bytes are provided.
    MissingBytes,

    /// The length is either too short or too long to represent a valid packet.
    BadLen(i32),

    /// The sequence number received does not match the expected value.
    BadSeq { expected: i32, received: i32 },

    /// The checksum of the packet does not match its expected value.
    BadCrc { computed: u32, received: u32 },

    /// A negative length was received, indicating a [transport-level error].
    /// The absolute value of this length behaves like an [HTTP status code]:
    ///
    /// * 404, if the Authorization Key used was not found, meaning that the
    ///   server is not aware of the key used by the client, so it cannot be
    ///   used to securely communicate with it.
    ///
    /// * 429, if too many transport connections are established to the same
    ///   IP address in a too-short lapse of time.
    ///
    /// [transport-level error]: https://core.telegram.org/mtproto/mtproto-transports#transport-errors
    /// [HTTP status code]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
    Status(i32),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        f.write_str("transport error: ")?;

        match self {
            MissingBytes => todo!("being removed"),

            BadLen(len) => write!(f, "bad len: {len}"),
            BadSeq { received, expected } => {
                write!(f, "bad seq: expected {expected}, received {received}")
            }
            BadCrc { computed, received } => {
                write!(
                    f,
                    "bad crc: computed {computed:08x}, received {received:08x}"
                )
            }
            Status(status) => {
                write!(f, "status code (negative len): {status}")
            }
        }
    }
}
