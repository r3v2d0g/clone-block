/**************************************************************************************************
 *                                                                                                *
 * This Source Code Form is subject to the terms of the Mozilla Public                            *
 * License, v. 2.0. If a copy of the MPL was not distributed with this                            *
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.                                       *
 *                                                                                                *
 **************************************************************************************************/

// ======================================== Documentation ======================================= \\

//! A very simple macro that clones a list of variables before calling an expression.
//!
//! Based on this tweet: <https://twitter.com/untitaker/status/1299812136202493953>
//!
//! ## Example
//!
//! ```rust
//! use clone_block::clone;
//! use std::thread;
//!
//! let foo = "foo".to_string();
//!
//! let thread = thread::spawn(
//!     clone!(foo; move || {
//!         let foobar = format!("{}bar", foo);
//!         foobar
//!     })
//! );
//!
//! let foobar = thread.join();
//! let foobaz = format!("{}baz", foo);
//! ```

// ======================================== macro_rules! ======================================== \\

#[macro_export]
/// ## Example
///
/// ```rust
/// use clone_block::clone;
/// use std::thread;
///
/// let foo = "foo".to_string();
///
/// let thread = thread::spawn(
///     clone!(foo; move || {
///         let foobar = format!("{}bar", foo);
///         foobar
///     })
/// );
///
/// let foobar = thread.join();
/// let foobaz = format!("{}baz", foo);
/// ```
macro_rules! clone {
    ($($var:ident),+ ; $expr:expr) => {{
        $(let $var = $var.clone();)+

        $expr
    }};
}
