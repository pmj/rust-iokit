fn main()
{
	let notify_port = iokit::NotificationPort::new();
	notify_port.set_dispatch_queue(dispatch::Queue::main());

	let matching_dict = iokit::service_matching("IOUSBHostDevice");

	let mut matching_notification = iokit::service_add_matching_notification(
		&notify_port, IOKit_sys::kIOMatchedNotification(), matching_dict,
		|service: IOKit_sys::io_object_t|
		{
			let mut path_buffer : Vec<libc::c_char> = vec![0; 512]; // io_string_t
			let path =
				unsafe
				{
					IOKit_sys::IORegistryEntryGetPath(service, IOKit_sys::kIOServicePlane(), path_buffer.as_mut_ptr());
					let path_cstr = std::ffi::CStr::from_ptr(path_buffer.as_ptr());
					path_cstr.to_str().unwrap().to_owned()
				};
			println!("{}",path);
		}).unwrap();
	
	matching_notification.start_handling_matches();
	
	unsafe
	{
		dispatch::ffi::dispatch_main();
	}
}