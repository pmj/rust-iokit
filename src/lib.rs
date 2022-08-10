extern crate IOKit_sys as io;
extern crate CoreFoundation_sys;
extern crate core_foundation as cf;
extern crate libc;

use cf::base::TCFType;
use cf::dictionary::CFMutableDictionary;

use std::ffi::CString;

mod notificationport;
mod ioobject;
pub use notificationport::*;
pub use self::ioobject::*;

pub fn service_matching(class_name: &str) -> CFMutableDictionary
{
	let service_class_name = CString::new(class_name).unwrap();
	unsafe
	{
		CFMutableDictionary::wrap_under_create_rule(io::IOServiceMatching(service_class_name.as_ptr()) as *mut cf::dictionary::__CFDictionary )
	}
}

pub struct MatchingNotification<'port>
{
	match_iterator: io::io_iterator_t,
	callback: Box<Box<dyn FnMut(IOServiceRef)>>,
	notification_port: std::marker::PhantomData<&'port NotificationPort>,
}

impl <'port> MatchingNotification<'port>
{
	pub fn start_handling_matches(&mut self)
	{
		loop
		{
			unsafe
			{
				let obj = io::IOIteratorNext(self.match_iterator);
				if obj == io::IO_OBJECT_NULL
				{
					break;
				}
				(**self.callback)(IOServiceRef::with_owned_service(obj));
			}
		}
	}
}

impl<'port> Drop for MatchingNotification<'port>
{
	fn drop(&mut self)
	{
		unsafe
		{
			io::IOObjectRelease(self.match_iterator);
		}
	}
	
}

extern "C" fn matching_service_notification_callback(refcon: *mut ::libc::c_void, iterator: io::io_iterator_t)
{
  let closure: &mut Box<dyn FnMut(io::io_service_t)> = unsafe { std::mem::transmute(refcon) };

	loop
	{
		unsafe
		{
			let obj = io::IOIteratorNext(iterator);
			if obj == io::IO_OBJECT_NULL
			{
				break;
			}
			closure(obj);
			io::IOObjectRelease(obj);
		}
	}
}

pub fn service_add_matching_notification<'port, F>(port: &'port NotificationPort, notification_type: *const libc::c_char, matching_dict: CFMutableDictionary, match_callback: F)
	-> Result<MatchingNotification<'port>, io::IOReturn>
	where F: FnMut(IOServiceRef) -> (),
	      F: 'static
{
	let mut iterator : io::io_iterator_t = io::IO_OBJECT_NULL;

	let callback : Box<Box<dyn FnMut(IOServiceRef)>> = Box::new(Box::new(match_callback));
	let callback_ptr : *const Box<dyn FnMut(IOServiceRef)> = &*callback;
	let result = unsafe { io::IOServiceAddMatchingNotification(port.as_ptr(), notification_type, matching_dict.as_concrete_TypeRef() as  *const CoreFoundation_sys::__CFDictionary, matching_service_notification_callback, callback_ptr as *mut libc::c_void, &mut iterator) };
	std::mem::forget(matching_dict);

	if result == io::kIOReturnSuccess
	{
		Ok(MatchingNotification{ match_iterator: iterator, callback: callback, notification_port: std::marker::PhantomData })
	}
	else
	{
		Err(result)
	}
}
