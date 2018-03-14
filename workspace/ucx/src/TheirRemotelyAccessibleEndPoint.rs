// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An end point.
/// *MUST* be used inside as `Rc<RefCell<Endpoint<E, A>>`.
pub struct TheirRemotelyAccessibleEndPoint<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress>
{
	handle: ucp_ep_h,
	user_data_and_peer_failure_error_handler: E,
	parent_worker: Worker,
	_their_remote_address: Rc<A>, // We *MUST* hold a reference to this, otherwise the data in `end_point_parameters` contains raw pointers to socket address structures that may have been dropped.
	end_point_parameters: ucp_ep_params_t,
}

impl<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress> Drop for TheirRemotelyAccessibleEndPoint<E, A>
{
	// Dropping because there are no more Rc strong references.
	#[inline(always)]
	fn drop(&mut self)
	{
		#[inline(always)]
		fn drop_user_data<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress>(user_data: *mut c_void)
		{
			let weak: Weak<TheirRemotelyAccessibleEndPoint<E, A>> = unsafe { transmute(user_data) };
			drop(weak);
		}
		
		// Never properly initialized.
		if self.handle.is_null()
		{
			let user_data = self.end_point_parameters.user_data;
			if !user_data.is_null()
			{
				drop_user_data::<E, A>(user_data)
			}
		}
		// Initialized and in-use.
		else
		{
			// We need to modify the end-point, and remove the user data (ie set it to null).
			// So any callbacks from UCX now fail.
			
			// Modify the end point to release the user_data and error_handler so we can free them.
			let user_data_original = self.end_point_parameters.user_data;
			self.end_point_parameters.user_data = null_mut();
			self.end_point_parameters.err_handler = ucp_err_handler_t
			{
				cb: None,
				arg: null_mut(),
			};
			let change_user_data_status_pointer = unsafe { ucp_ep_modify_nb(self.debug_assert_handle_is_valid(), &self.end_point_parameters) };
			
			// We discard any errors; there's nothing we can do with them.
			#[allow(unused_must_use)]
			{
				self.parent_worker.block_until_non_blocking_request_is_complete(change_user_data_status_pointer);
			}
			
			// Drop the weak reference in user data.
			drop_user_data::<E, A>(user_data_original);
			
			let close_status_pointer = unsafe { ucp_ep_close_nb(self.debug_assert_handle_is_valid(), ucp_ep_close_mode::UCP_EP_CLOSE_MODE_FLUSH as u32) };
			
			// We discard any errors; there's nothing we can do with them.
			#[allow(unused_must_use)]
			{
				self.parent_worker.block_until_non_blocking_request_is_complete(close_status_pointer);
			}
			
			self.handle = null_mut();
		}
	}
}

impl<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress> Debug for TheirRemotelyAccessibleEndPoint<E, A>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress> PrintInformation for TheirRemotelyAccessibleEndPoint<E, A>
{
	const DebugName: &'static str = "TheirRemotelyAccessibleEndPointEndPoint";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { ucp_ep_print_info(self.debug_assert_handle_is_valid(), stream) };
	}
}

/// Implemented only for behaviour suitable for accessing remote servers.
impl<E: EndPointPeerFailureErrorHandler> TheirRemotelyAccessibleEndPoint<E, TheirRemotelyAccessibleServerEndPointAddress>
{
	/// Sends a message on a stream.
	///
	/// The provided message is not safe to re-use or write-to until this request has completed.
	///
	/// For a `callback_when_finished_or_cancelled` that does nothing, use `::ucx::callbacks::send_callback_is_ignored`.
	/// `request` should not be freed inside the `callback_when_finished_or_cancelled`.
	///
	/// If a returned `SendingStreamNonBlockingRequest` is neither cancelled or completed (ie it falls out of scope) then the request will be cancelled and the `message` dropped.
	#[inline(always)]
	pub fn stream_send_non_blocking_ucx_allocated<'worker, M: Message>(&'worker self, message: M, callback_when_finished_or_cancelled: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t)) -> Result<NonBlockingRequestCompletedOrInProgress<M, SendingStreamNonBlockingRequest<'worker, M>>, ErrorCodeWithMessage<M>>
	{
		let status_pointer = unsafe { ucp_stream_send_nb(self.debug_assert_handle_is_valid(), message.address().as_ptr() as *const c_void, message.count(), message.data_type_descriptor(), Some(callback_when_finished_or_cancelled), ReservedForFutureUseFlags) };
		
		match self.parent_worker.parse_status_pointer(status_pointer)
		{
			Ok(non_blocking_request_completed_or_in_progress) => match non_blocking_request_completed_or_in_progress
			{
				Completed(()) => Ok(Completed(message)),
				
				InProgress(non_blocking_request_in_progress) => Ok(InProgress(SendingStreamNonBlockingRequest::new(non_blocking_request_in_progress, message))),
			},
			
			Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message))
		}
	}
	
	/// Receives a message from a stream.
	///
	/// If `wait_for_all_data` is true, then the receive request will not complete until all of the requested data is received and is in `message`.
	///
	/// If `wait_for_all_data` is not specified, then less than a complete amount of data may be received, but what is received will be aligned to message's element size, ie there may be less items.
	///
	/// It is probably appropriate to use `wait_for_all_data` when message is a `GenericMessage` (UCX's documentation is lacking on this point).
	///
	/// The provided message is not safe to re-use or write-to until this request has completed.
	///
	/// For a `callback_when_finished_or_cancelled` that does nothing, use `::ucx::callbacks::stream_receive_callback_is_ignored`.
	/// `request` should not be freed inside the `callback_when_finished_or_cancelled`.
	///
	/// If a returned `SendingStreamNonBlockingRequest` is neither cancelled or completed (ie it falls out of scope) then the request will be cancelled and the `message` dropped.
	#[inline(always)]
	pub fn stream_receive_non_blocking_ucx_allocated<'worker, M: Message>(&'worker self, message: M, wait_for_all_data: bool, callback_when_finished_or_cancelled: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t, length: usize)) -> Result<NonBlockingRequestCompletedOrInProgress<M, ReceivingStreamNonBlockingRequest<'worker, M>>, ErrorCodeWithMessage<M>>
	{
		// ?length
		let flags = if wait_for_all_data
		{
			UCP_STREAM_RECV_FLAG_WAITALL
		}
		else
		{
			0
		};
		
		let mut length = unsafe { uninitialized() };
		let status_pointer = unsafe { ucp_stream_recv_nb(self.debug_assert_handle_is_valid(), message.address().as_ptr() as *mut c_void, message.count(), message.data_type_descriptor(), Some(callback_when_finished_or_cancelled), &mut length, flags) };
		
		match self.parent_worker.parse_status_pointer(status_pointer)
		{
			Ok(non_blocking_request_completed_or_in_progress) => match non_blocking_request_completed_or_in_progress
			{
				Completed(()) => Ok(Completed(message)),
				
				InProgress(non_blocking_request_in_progress) => Ok(InProgress(ReceivingStreamNonBlockingRequest::new(non_blocking_request_in_progress, message))),
			},
			
			Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message))
		}
	}
	
	/*
	
	Stream
	
	
	#[link_name = "\u{1}_ucp_stream_data_release"] pub fn ucp_stream_data_release(ep: ucp_ep_h, data: *mut c_void);
	#[link_name = "\u{1}_ucp_stream_recv_data_nb"] pub fn ucp_stream_recv_data_nb(ep: ucp_ep_h, length: *mut usize) -> ucs_status_ptr_t;
	
	
	
	#[link_name = "\u{1}_ucp_stream_recv_request_test"] pub fn ucp_stream_recv_request_test(request: *mut c_void, length_p: *mut usize) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_stream_worker_poll"] pub fn ucp_stream_worker_poll(worker: ucp_worker_h, poll_eps: *mut ucp_stream_poll_ep_t, max_eps: usize, flags: c_uint) -> isize;
	
	*/
}

/// Implemented only for behaviour suitable for accessing remote workers.
impl<E: EndPointPeerFailureErrorHandler> TheirRemotelyAccessibleEndPoint<E, TheirRemotelyAccessibleWorkerEndPointAddress>
{
	/// Can be called more than once per end point.
	/// Think of the world as multiple threads (worker), each of which is connected to a remote peer (end point), each of which is connected to zero or more remote memory regions.
	/// Remote memory regions are not needed for tagged tagged_messages and streams.
	#[inline(always)]
	pub fn use_remote_memory_region(this: &Rc<RefCell<Self>>, their_remotely_accessible_memory_address: TheirRemotelyAccessibleMemoryAddress) -> Result<TheirRemotelyAccessibleMemory<E>, ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		let status = unsafe { ucp_ep_rkey_unpack(this.borrow().debug_assert_handle_is_valid(), their_remotely_accessible_memory_address.0.as_ptr() as *mut _, &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok
			(
				TheirRemotelyAccessibleMemory
				{
					handle,
					parent_end_point: this.clone(),
				}
			),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	/// Sends a tagged message, using a user_allocated_non_blocking_request that can have been stack-allocated.
	///
	/// The provided message is not safe to re-use or write-to until this request has completed.
	///
	/// Does not take a callback.
	///
	/// Returns Ok(true, message buffer)
	#[inline(always)]
	pub fn non_blocking_send_tagged_message_user_allocated<'worker, M: Message>(&'worker self, message: M, tag: TagValue, user_allocated_non_blocking_request: UserAllocatedNonBlockingRequest) -> Result<NonBlockingRequestCompletedOrInProgress<M, SendingTaggedMessageNonBlockingRequest<'worker, M, UserAllocatedNonBlockingRequest>>, ErrorCodeWithMessage<M>>
	{
		let status = unsafe { ucp_tag_send_nbr(self.debug_assert_handle_is_valid(), message.address().as_ptr() as *const c_void, message.count(), message.data_type_descriptor(), tag.0, user_allocated_non_blocking_request.non_null_pointer().as_ptr() as *mut c_void) };
		
		use self::Status::*;
		use self::NonBlockingRequestCompletedOrInProgress::*;
		
		match status.parse()
		{
			IsOk => Ok(Completed(message)),
			
			OperationInProgress => Ok(InProgress(SendingTaggedMessageNonBlockingRequest::new(WorkerWithNonBlockingRequest::new(&self.parent_worker, user_allocated_non_blocking_request), message))),
			
			Error(error_code) => Err(ErrorCodeWithMessage::new(error_code, message)),
			
			UnknownErrorCode(unknown_error_code) => panic!("UnknownErrorCode '{}'", unknown_error_code),
		}
	}
	
	/// Sends a tagged message.
	///
	/// It is preferable to use `non_blocking_send_tagged_message_user_allocated` instead as it is more efficient and has an easier API to work with.
	///
	/// The provided message is not safe to re-use or write-to until this request has completed.
	///
	/// For a `callback_when_finished_or_cancelled` that does nothing, use `::ucx::callbacks::send_callback_is_ignored`.
	/// `request` should not be freed inside the `callback_when_finished_or_cancelled`.
	///
	/// If a returned `SendingStreamNonBlockingRequest` is neither cancelled or completed (ie it falls out of scope) then the request will be cancelled and the `message` dropped.
	#[inline(always)]
	pub fn non_blocking_send_tagged_message_ucx_allocated<'worker, M: Message>(&'worker self, message: M, tag: TagValue, callback_when_finished_or_cancelled: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t)) -> Result<NonBlockingRequestCompletedOrInProgress<M, SendingTaggedMessageNonBlockingRequest<'worker, M>>, ErrorCodeWithMessage<M>>
	{
		let status_pointer = unsafe { ucp_tag_send_nb(self.debug_assert_handle_is_valid(), message.address().as_ptr() as *const c_void, message.count(), message.data_type_descriptor(), tag.0, Some(callback_when_finished_or_cancelled)) };

		match self.parent_worker.parse_status_pointer(status_pointer)
		{
			Ok(non_blocking_request_completed_or_in_progress) => match non_blocking_request_completed_or_in_progress
			{
				Completed(()) => Ok(Completed(message)),
				
				InProgress(non_blocking_request_in_progress) => Ok(InProgress(SendingTaggedMessageNonBlockingRequest::new(non_blocking_request_in_progress, message))),
			},
			
			Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message))
		}
	}
	
	/// Sends a tagged message and only completes when the recipient has matched its tag (but not necessarily received its contents).
	///
	/// Never completes immediately.
	///
	/// The provided message is not safe to re-use or write-to until this request has completed.
	///
	/// For a `callback_when_finished_or_cancelled` that does nothing, use `::ucx::callbacks::send_callback_is_ignored`.
	/// `request` should not be freed inside the `callback_when_finished_or_cancelled`.
	///
	/// If a returned `SendingStreamNonBlockingRequest` is neither cancelled or completed (ie it falls out of scope) then the request will be cancelled and the `message` dropped.
	#[inline(always)]
	pub fn non_blocking_send_tagged_message_completing_only_when_recipient_has_matched_its_tag<'worker, M: Message>(&'worker self, message: M, tag: TagValue, callback_when_finished_or_cancelled: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t)) -> Result<SendingTaggedMessageNonBlockingRequest<'worker, M>, ErrorCodeWithMessage<M>>
	{
		let status_pointer = unsafe { ucp_tag_send_sync_nb(self.debug_assert_handle_is_valid(), message.address().as_ptr() as *const c_void, message.count(), message.data_type_descriptor(), tag.0, Some(callback_when_finished_or_cancelled)) };

		match self.parent_worker.parse_status_pointer(status_pointer)
		{
			Ok(non_blocking_request_completed_or_in_progress) => match non_blocking_request_completed_or_in_progress
			{
				Completed(()) => panic!("API documentation notes that completion never happens initially"),
				
				InProgress(non_blocking_request_in_progress) => Ok(SendingTaggedMessageNonBlockingRequest::new(non_blocking_request_in_progress, message)),
			},
			
			Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message))
		}
	}
}

impl<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress> TheirRemotelyAccessibleEndPoint<E, A>
{
	/// A non-blocking flush.
	///
	/// Potentially quite expensive.
	///
	/// `request` points to memory that was previously initialized using the `NonBlockingRequestMemoryCustomization` trait, which is a type parameter of `MemoryCustomization` on the `ApplicationContext`.
	///
	/// For a `callback_when_finished_or_cancelled` that does nothing, use `::ucx::callbacks::send_callback_is_ignored`.
	/// `request` should not be freed inside the `callback_when_finished_or_cancelled`.
	///
	/// Returns `Ok(())` if initiated and is already complete.
	/// Returns `Ok(WorkerWithNonBlockingRequest)` if initiated but not complete.
	/// Returns `Err(NoResourcesAreAvailableToInitiateTheOperation`) if no resources are available; it may be possible to try again.
	/// Returns `Err` for other failures, the cause of which isn't clear.
	///
	#[inline(always)]
	pub fn non_blocking_flush<'worker>(&'worker self, callback_when_finished_or_cancelled: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t)) -> Result<NonBlockingRequestCompletedOrInProgress<(), WorkerWithNonBlockingRequest<'worker>>, ErrorCode>
	{
		// NOTE: Despite the signature of `ucp_ep_flush_nb`, the callback_when_finished_or_cancelled is *NOT* optional.
		let status_pointer = unsafe { ucp_ep_flush_nb(self.debug_assert_handle_is_valid(), ReservedForFutureUseFlags, Some(callback_when_finished_or_cancelled)) };
		
		self.parent_worker.parse_status_pointer(status_pointer)
	}
	
	/// A blocking flush.
	///
	/// Potentially very expensive.
	#[inline(always)]
	pub fn blocking_flush(&self) -> Result<(), ErrorCode>
	{
		self.parent_worker.block_until_non_blocking_request_is_complete(unsafe { ucp_ep_flush_nb(self.debug_assert_handle_is_valid(), ReservedForFutureUseFlags, Some(send_callback_is_ignored)) })
	}
	
	#[inline(always)]
	pub(crate) fn new(peer_failure_error_handler: E, their_remote_address: &Rc<A>, guarantee_that_send_requests_are_always_completed_successfully_or_error: bool, parent_worker: &Worker) -> Result<Rc<RefCell<Self>>, ErrorCode>
	{
		#[inline(always)]
		fn populated_by_their_remote_address<T>() -> T
		{
			unsafe { zeroed() }
		}
		
		use self::ucp_err_handling_mode_t::*;
		
		let end_point = Rc::new
		(
			RefCell::new
			(
				Self
				{
					handle: null_mut(),
					user_data_and_peer_failure_error_handler: peer_failure_error_handler,
					parent_worker: parent_worker.clone(),
					_their_remote_address: their_remote_address.clone(),
					end_point_parameters: their_remote_address.populate_end_point_parameters
					(
						ucp_ep_params_t
						{
							field_mask: (ucp_ep_params_field::ERR_HANDLING_MODE | ucp_ep_params_field::ERR_HANDLER | ucp_ep_params_field::USER_DATA).0 as u64,
							
							address: populated_by_their_remote_address(),
							
							err_mode: if guarantee_that_send_requests_are_always_completed_successfully_or_error
							{
								UCP_ERR_HANDLING_MODE_PEER
							}
							else
							{
								UCP_ERR_HANDLING_MODE_NONE
							},
							
							err_handler: ucp_err_handler
							{
								cb: Some(Self::peer_failure_error_callback),
								arg: null_mut(), // Is overridden by `ucp_ep_params_t.user_data`.
							},
							
							user_data: null_mut(),
							
							flags: populated_by_their_remote_address(),
							
							sockaddr: populated_by_their_remote_address(),
						}
					),
				}
			)
		);
		Self::assign_user_data_to_self(&end_point);
		
		(*end_point).borrow_mut().connect()?;
		
		Ok(end_point)
	}
	
	#[inline(always)]
	fn connect(&mut self) -> Result<(), ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		let result = unsafe { ucp_ep_create(self.parent_worker.handle, &self.end_point_parameters, &mut handle) };
		let status = result.parse();
		
		use self::Status::*;
		
		match status
		{
			IsOk =>
			{
				self.handle = handle;
				Ok(())
			}
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("Unexpected status '{:?}'", unexpected)
		}
	}
	
	// Yes, this is horrible, but how else does one pack a Weak<TheirRemotelyAccessibleEndPointEndPoint<E>> into a C FFI `user_data` field of type void*?
	// (Actually, by possibly using a user_data = Box<Weak<TheirRemotelyAccessibleEndPointEndPoint<E>>>::into_raw()... but that involves indirection).
	#[inline(always)]
	pub(crate) fn assign_user_data_to_self(this: &Rc<RefCell<Self>>)
	{
		let mut end_point = this.borrow_mut();
		(*end_point).end_point_parameters.user_data = unsafe { transmute(Rc::downgrade(this)) };
	}
	
	// Yes, this is also horrible.
	// `user_data` is an aliased value - there can be multiple copies for one logical `Weak<Self>`.
	#[inline(always)]
	pub(crate) fn end_point_from_user_data(user_data: *mut c_void, handle: ucp_ep_h) -> Option<Rc<RefCell<Self>>>
	{
		// This can only happen during the drop of the end point (we tell UCX to modify the end point and give it null user_data).
		if user_data.is_null()
		{
			return None;
		}
		
		let weak: Weak<RefCell<Self>> = unsafe { transmute(user_data) };
		let possibly_strong = weak.upgrade();
		forget(weak);
		
		if let Some(strong) = possibly_strong
		{
			// Either not yet initialized (so no errors should have been raised).
			// Or has been dropped but not freed.
			let our_handle = strong.borrow().handle;
			if our_handle.is_null()
			{
				None
			}
			else
			{
				debug_assert!(handle == our_handle);
				Some(strong)
			}
		}
		else
		{
			None
		}
	}
	
	// Yes, this is another horrible piece of code.
	#[inline(always)]
	unsafe extern "C" fn peer_failure_error_callback(user_data: *mut c_void, ep: ucp_ep_h, status: ucs_status_t)
	{
		if let Some(this) = Self::end_point_from_user_data(user_data, ep)
		{
			this.borrow_mut().user_data_and_peer_failure_error_handler.peer_failure(status.error_code_or_panic())
		}
	}
	
	#[inline(always)]
	fn debug_assert_handle_is_valid(&self) -> ucp_ep_h
	{
		let handle = self.handle;
		debug_assert!(!handle.is_null(), "handle is null");
		handle
	}
}
