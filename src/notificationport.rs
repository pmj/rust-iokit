
extern crate IOKit_sys as io;
extern crate dispatch;

use ::std::mem::forget;

pub struct NotificationPort
{
	owned_port_ref: *mut io::IONotificationPort
}

impl NotificationPort
{
	pub fn new() -> NotificationPort
	{
		unsafe
		{
			NotificationPort{ owned_port_ref: io::IONotificationPortCreate(io::kIOMasterPortDefault) }
		}
	}
	pub fn destroy(port: NotificationPort)
	{
		unsafe
		{
			io::IONotificationPortDestroy(port.owned_port_ref);
			forget(port);
		}
	}
	pub fn as_ptr(&self) -> io::IONotificationPortRef
	{
		return self.owned_port_ref;
	}

	pub fn set_dispatch_queue(&self, queue: dispatch::Queue)
	{
		unsafe
		{
			io::IONotificationPortSetDispatchQueue(self.as_ptr(), queue.as_ptr());
		}
	}
}

impl Drop for NotificationPort
{
	fn drop(&mut self)
	{
		unsafe
		{
			io::IONotificationPortDestroy(self.owned_port_ref)
		}
	}	
}
