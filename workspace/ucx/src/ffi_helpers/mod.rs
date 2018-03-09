// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::libc::c_char;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::fmt::Debug;


include!("FromCBool.rs");
include!("Integer.rs");
include!("null_or_empty_c_string.rs");
include!("ReservedForFutureUseFlags.rs");
include!("ToCBool.rs");