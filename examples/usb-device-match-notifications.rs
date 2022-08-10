extern crate iokit;
use iokit::*;

fn main()
{
	let notify_port = iokit::NotificationPort::new();
	notify_port.set_dispatch_queue(dispatch::Queue::main());

	let matching_dict = iokit::service_matching("IOUSBHostDevice");

	let mut matching_notification = iokit::service_add_matching_notification(
		&notify_port, IOKit_sys::kIOMatchedNotification(), matching_dict,
		|service: iokit::IOServiceRef|
		{
			let path = service.get_path(iokit::RegistryPlane::Service);
			println!("{}",path.unwrap());
			service.release();
		}).unwrap();
	
	matching_notification.start_handling_matches();
	
	unsafe
	{
		dispatch::ffi::dispatch_main();
	}
}