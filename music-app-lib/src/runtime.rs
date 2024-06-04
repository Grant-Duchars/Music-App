//! # Examples
//! ```
//! use music_app_lib::runtime::to_words;
//! // Formats both non-plural ...
//! assert_eq!("1 sec", to_words(1));
//! assert_eq!("1 min, 1 sec", to_words(61));
//! assert_eq!("1 hour, 1 min, 1 sec", to_words(3_661));
//! assert_eq!("1 day, 1 hour, 1 min, 1 sec", to_words(90_061));
//! assert_eq!("1 week, 1 day, 1 hour, 1 min, 1 sec", to_words(694_861));
//! assert_eq!("1 year, 1 week, 1 day, 1 hour, 1 min, 1 sec", to_words(32_144_461));
//! // ... and plural quantities ...
//! assert_eq!("59 secs", to_words(59));
//! assert_eq!("59 mins, 59 secs", to_words(3_599));
//! assert_eq!("23 hours, 59 mins, 59 secs", to_words(86_399));
//! assert_eq!("6 days, 23 hours, 59 mins, 59 secs", to_words(604_799));
//! assert_eq!("51 weeks, 6 days, 23 hours, 59 mins, 59 secs", to_words(31_449_599));
//! assert_eq!("999 years, 51 weeks, 6 days, 23 hours, 59 mins, 59 secs", to_words(31_449_599_999));
//! // ... and skips any empty quantities.
//! assert_eq!("4 weeks, 6 hours, 42 mins, 12 secs", to_words(2_443_332))
//! // ... and that ... is when the world ... will end.
//! ```
//! ```
//! use music_app_lib::runtime::to_digital;
//!
//! assert_eq!("0:01", to_digital(1));
//! assert_eq!("1:01", to_digital(61));
//! assert_eq!("1:01:01", to_digital(3_661));
//! assert_eq!("1:01:01:01", to_digital(90_061));
//! assert_eq!("1:01:01:01:01", to_digital(694_861));
//! assert_eq!("1:01:01:01:01:01", to_digital(32_144_461));
//!
//! assert_eq!("0:59", to_digital(59));
//! assert_eq!("59:59", to_digital(3_599));
//! assert_eq!("23:59:59", to_digital(86_399));
//! assert_eq!("6:23:59:59", to_digital(604_799));
//! assert_eq!("51:06:23:59:59", to_digital(31_449_599));
//! assert_eq!("999:51:06:23:59:59", to_digital(31_449_599_999));
//! ```

/// Calculates an album or song's runtime/length from a given number \
/// # Examples
/// ```
/// use music_app_lib::runtime::calc_runtime;
///
/// assert_eq!([0, 0, 0, 0, 0, 1], calc_runtime(1));
/// assert_eq!([0, 0, 0, 0, 1, 1], calc_runtime(61));
/// assert_eq!([0, 0, 0, 1, 1, 1], calc_runtime(3_661));
/// assert_eq!([0, 0, 1, 1, 1, 1], calc_runtime(90_061));
/// assert_eq!([0, 1, 1, 1, 1, 1], calc_runtime(694_861));
/// assert_eq!([1, 1, 1, 1, 1, 1], calc_runtime(32_144_461));
///
/// assert_eq!([000, 00, 0, 00, 00, 59], calc_runtime(59));
/// assert_eq!([000, 00, 0, 00, 59, 59], calc_runtime(3_599));
/// assert_eq!([000, 00, 0, 23, 59, 59], calc_runtime(86_399));
/// assert_eq!([000, 00, 6, 23, 59, 59], calc_runtime(604_799));
/// assert_eq!([000, 51, 6, 23, 59, 59], calc_runtime(31_449_599));
/// assert_eq!([999, 51, 6, 23, 59, 59], calc_runtime(31_449_599_999));
/// ```
pub fn calc_runtime(num: usize) -> [usize; 6] {
    // Conversion Constants    [Year, Week, Day, Hour, Minute]
    const CONSTS: [usize; 6] = [31_449_600, 604_800, 86_400, 3_600, 60, 1];
    let mut num = num;
    let mut runtime = CONSTS.iter().map(move |c| {
        let conv = num / c;
        num %= c;
        conv
    });
    [
        runtime.next().unwrap(),
        runtime.next().unwrap(),
        runtime.next().unwrap(),
        runtime.next().unwrap(),
        runtime.next().unwrap(),
        runtime.next().unwrap(),
    ]
}

/// Converts a number to a string based on how many seconds it represents \
/// # Examples
/// ```
/// use music_app_lib::runtime::to_words;
/// // Formats both non-plural ...
/// assert_eq!("1 sec", to_words(1));
/// assert_eq!("1 min, 1 sec", to_words(61));
/// assert_eq!("1 hour, 1 min, 1 sec", to_words(3_661));
/// assert_eq!("1 day, 1 hour, 1 min, 1 sec", to_words(90_061));
/// assert_eq!("1 week, 1 day, 1 hour, 1 min, 1 sec", to_words(694_861));
/// assert_eq!("1 year, 1 week, 1 day, 1 hour, 1 min, 1 sec", to_words(32_144_461));
/// // ... and plural quantities ...
/// assert_eq!("59 secs", to_words(59));
/// assert_eq!("59 mins, 59 secs", to_words(3_599));
/// assert_eq!("23 hours, 59 mins, 59 secs", to_words(86_399));
/// assert_eq!("6 days, 23 hours, 59 mins, 59 secs", to_words(604_799));
/// assert_eq!("51 weeks, 6 days, 23 hours, 59 mins, 59 secs", to_words(31_449_599));
/// assert_eq!("999 years, 51 weeks, 6 days, 23 hours, 59 mins, 59 secs", to_words(31_449_599_999));
/// // ... and skips any empty quantities.
/// assert_eq!("4 weeks, 6 hours, 42 mins, 12 secs", to_words(2_443_332))
/// // ... and that ... is when the world ... will end.
/// ```
pub fn to_words(num: usize) -> String {
    let words = ["year", "week", "day", "hour", "min", "sec"];
    let runtime = calc_runtime(num);
    let runtime = runtime
        .into_iter()
        .zip(words)
        .map(|(num, word)| {
            if num != 0 {
                Some(format!("{num} {word}{}", if num == 1 { "" } else { "s" }))
            } else {
                None
            }
        })
        .rev();
    let mut output = String::new();
    for num in runtime.flatten() {
        output = if output.as_str() != "" {
            format!("{num}, {output}")
        } else {
            num
        }
    }
    output
}

/// Converts a number to a string based on how many seconds it represents \
/// # Examples
/// ```
/// use music_app_lib::runtime::to_digital;
///
/// assert_eq!("0:01", to_digital(1));
/// assert_eq!("1:01", to_digital(61));
/// assert_eq!("1:01:01", to_digital(3_661));
/// assert_eq!("1:01:01:01", to_digital(90_061));
/// assert_eq!("1:01:01:01:01", to_digital(694_861));
/// assert_eq!("1:01:01:01:01:01", to_digital(32_144_461));
///
/// assert_eq!("0:59", to_digital(59));
/// assert_eq!("59:59", to_digital(3_599));
/// assert_eq!("23:59:59", to_digital(86_399));
/// assert_eq!("6:23:59:59", to_digital(604_799));
/// assert_eq!("51:06:23:59:59", to_digital(31_449_599));
/// assert_eq!("999:51:06:23:59:59", to_digital(31_449_599_999));
/// ```
pub fn to_digital(num: usize) -> String {
    use std::fmt::Write;
    let mut runtime = calc_runtime(num).into_iter().zip([':'; 6]).fold(
        String::new(),
        |mut output, (num, delim)| {
            let _ = write!(output, "{num:0>2}{delim}");
            output
        },
    );
    runtime.pop();
    let runtime = runtime.trim_start_matches(['0', ':']);
    if runtime.len() <= 2 {
        format!("0:{runtime:0>2}")
    } else {
        runtime.into()
    }
}
