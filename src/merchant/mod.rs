/***
 * Copyright (c) 2025 Rafael Fernández López <ereslibre@curried.software>
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy,
 * modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 ***/

use crate::client::Client;

/// The error type returned by every Merchant API call. The Merchant API reports
/// its error `code` as a string identifier (per the `Error-v2` schema), so it
/// is pinned to `errors::Error<String>` — callers read `code: Option<String>`
/// directly and never see a numeric Business-style code.
pub type Error = crate::errors::Error<String>;
/// The result type returned by every Merchant API call. See [`Error`].
pub type ApiResult<T> = crate::errors::ApiResult<T, String>;

pub mod apple_pay;
pub mod client;
pub mod customers;
pub mod disputes;
pub mod generated;
pub mod locations;
pub mod orders;
pub mod other;
pub mod payments;
pub mod payouts;
pub mod report_runs;
pub mod subscriptions;
pub mod webhooks;

pub mod v2026_04_20;

pub use v2026_04_20 as latest;
