// This file is part of ucx-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx-sys/master/COPYRIGHT. No part of ucx-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx-sys/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![feature(static_nobundle)]
#![feature(untagged_unions)]


#[cfg(target_os = "linux")] extern crate libnuma_sys;
#[cfg(target_os = "linux")] extern crate mlnx_ofed_libibverbs_sys;
#[cfg(target_os = "linux")] extern crate mlnx_ofed_libmlx5_sys;


#[cfg(target_os = "linux")] include!("bindgen/lib.rs");
