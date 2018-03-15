// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::ffi_helpers::FromCBool;
use super::ffi_helpers::ReservedForFutureUseFlags;
use super::ffi_helpers::ToCBool;
use super::tagged_messages::TagMatcher;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::size_t;
use ::libc::ssize_t;
use ::libc::uint64_t;
use ::std::cell::UnsafeCell;
use ::std::ptr::null_mut;
use ::ucx_sys::*;


include!("CallbackQueue.rs");
include!("CommunicationInterfaceContext.rs");
include!("CompletionHandleHelper.rs");
include!("ReceiveDescriptor.rs");
include!("RemoteEndPoint.rs");
include!("Worker.rs");


// search for `ep_put_short` to find 'implementors' of uct_iface_ops_t