// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::errors::*;
use self::ucs_status_t::*;
use ::std::ffi::CStr;
use ::std::mem::size_of_val;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::ucx_sys::*;


include!("ucs_cpu_set_tExt.rs");
include!("ucs_status_tExt.rs");
include!("ucs_status_ptr_tExt.rs");
include!("ZeroBasedHyperThreadIndex.rs");
