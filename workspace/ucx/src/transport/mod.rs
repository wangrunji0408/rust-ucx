// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::communication_interface_context::InterfaceFeaturesSupported;
use super::attributes::*;
use super::ffi_helpers::*;
use super::handle_drop_safeties::*;
use super::status::*;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::size_t;
use ::libc::ssize_t;
use ::libc::uint64_t;
use ::std::cell::UnsafeCell;
use ::std::mem::size_of_val;
use ::std::mem::uninitialized;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::sync::Arc;
use ::ucx_sys::*;


/// Communication interface context and supporting logic.
pub mod communication_interface_context;


/// Memory domains needed to create communication interface contexts.
pub mod memory_domain;


/// A progress engine is used to progress non-blocking operations.
pub mod progress_engine;


include!("ActiveMessageIdentifier.rs");
include!("AsynchronousContext.rs");
include!("CompletionHandleHelper.rs");
include!("ReceiveDescriptor.rs");
include!("RemoteEndPoint.rs");
