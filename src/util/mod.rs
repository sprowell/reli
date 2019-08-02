//! Provide some general helper functions that do not depend on any other module.
//!
//! ```text
//!           _ _     _
//!  _ __ ___| (_)___(_) ___  _ __
//! | '__/ _ \ | / __| |/ _ \| '_ \
//! | | |  __/ | \__ \ | (_) | | | |
//! |_|  \___|_|_|___/_|\___/|_| |_|
//! ```
//! The relision term rewriting library.
//!
//! # License
//!
//! Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//!
//! Licensed under the BSD 2-Clause license.  See the file LICENSE
//! that is part of this distribution.  This file may not be copied,
//! modified, or distributed except according to those terms.

/// Given a string and a "border" character, properly escape special characters in the string.
/// Special characters are as defined in Rust.
///
/// * tabulator (U+0009) becomes \t
/// * line feed (U+000A) becomes \a
/// * carriage return (U+000D) becomes \r
/// * backslash or reverse solidus (U+005C) becomes \\
/// * single quote is escaped as \'
/// * double quote is escaped as \"
/// 
/// Any printable ASCII character (0x20 - 0x7e) is left as-is.  All other
/// characters are escaped using one of two hexadecimal escape methods.  Other
/// characters are rendered using the Rust standard \u{HHHHHH}.
///
/// Finally, if the character specified by `border` is found, it is also escaped.
///
/// The escaped string is returned.  Note that hex digits are lower case.  The Boolean that
/// is returned indicates whether any changes were made to the string; that is, was anything
/// actually escaped?
pub fn escape(input: &String, border: char) -> (String, bool) {
    let mut output = String::new();
    let mut fixed = false;
    for ch in input.chars() {
        if ch == border {
            output.push_str("\\");
            output.push(border);
            fixed = true;
            continue;
        }
        let ech = &ch.escape_default().to_string();
        if ech.len() > 1 { fixed = true; }
        output.push_str(ech);
    }
    (output, fixed)
}
