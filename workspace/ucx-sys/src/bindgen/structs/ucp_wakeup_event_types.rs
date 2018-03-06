// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucp_wakeup_event_types(u32);

impl BitOr<ucp_wakeup_event_types> for ucp_wakeup_event_types
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucp_wakeup_event_types(self.0 | other.0)
	}
}

impl BitOrAssign for ucp_wakeup_event_types
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucp_wakeup_event_types)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucp_wakeup_event_types> for ucp_wakeup_event_types
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucp_wakeup_event_types(self.0 & other.0)
	}
}

impl BitAndAssign for ucp_wakeup_event_types
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucp_wakeup_event_types)
	{
		self.0 &= rhs.0;
	}
}

impl ucp_wakeup_event_types
{
	pub const AMO: Self = ucp_wakeup_event_types(2);
	pub const EDGE: Self = ucp_wakeup_event_types(65536);
	pub const RMA: Self = ucp_wakeup_event_types(1);
	pub const RX: Self = ucp_wakeup_event_types(2048);
	pub const TAG_RECV: Self = ucp_wakeup_event_types(8);
	pub const TAG_SEND: Self = ucp_wakeup_event_types(4);
	pub const TX: Self = ucp_wakeup_event_types(1024);
}