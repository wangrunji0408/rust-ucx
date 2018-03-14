// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A contiguous message.
#[derive(Debug)]
pub struct ContiguousMessage<'a, T: 'a + Debug>
{
	array: &'a [T],
	data_type_descriptor: ContiguousDataTypeDescriptor,
}

impl<'a, T: Debug> Message for ContiguousMessage<'a, T>
{
	#[inline(always)]
	fn address(&self) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked(self.array.as_ptr() as *mut _) }
	}
	
	#[inline(always)]
	fn count(&self) -> usize
	{
		self.array.len()
	}
	
	#[inline(always)]
	fn data_type_descriptor(&self) -> ucp_datatype_t
	{
		self.data_type_descriptor.to_ucp_datatype_t()
	}
}

impl<'a, T: Debug> ContiguousMessage<'a, T>
{
	/// Creates new instance.
	#[inline(always)]
	pub fn new(array: &'a [T]) -> Self
	{
		Self
		{
			array,
			data_type_descriptor: ContiguousDataTypeDescriptor::new(size_of::<T>() as u64),
		}
	}
}
