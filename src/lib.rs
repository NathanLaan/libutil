#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn println() {
        dbg_println!("Test Message");
    }
}

///
/// Debug `println!` macro that calls `println` with the specified message(s), and
/// prefixes a time stamp. Only prints out when the build is in debug mode.
///
#[macro_export]
macro_rules! dbg_println {
    // Match any number of argument tokens as a token tree
    ($($arg:tt)*) => {
        // Evaluates true only when compiled in debug mode
        #[cfg(debug_assertions)]
        {
            use time::{OffsetDateTime, format_description::well_known::Rfc3339};

            // Get the current local time and fall back to UTC
            let now = OffsetDateTime::now_local()
                .unwrap_or_else(|_| OffsetDateTime::now_utc());

            // Use RFC3339 format:  YYYY-MM-DDTTHH:MM::SSSSSZ
            let now_format = now.format(&Rfc3339)
                .unwrap_or_else(|_| "UNKNOWN".into());

            println!("[{}] {}", now_format, format!($($arg)*));
        }
    };
}
