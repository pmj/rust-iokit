extern crate IOKit_sys as io;

pub enum RegistryPlane
{
	Service
}

fn plane_cchar(plane : RegistryPlane) -> *const libc::c_char
{
	match plane
	{
		RegistryPlane::Service => io::kIOServicePlane()
	}
}

pub struct IOObjectBaseRef
{
	object : io::io_object_t
}

impl IOObjectBaseRef
{
	pub fn release(self)
	{
		if self.object != io::IO_OBJECT_NULL
		{
			unsafe
			{
				io::IOObjectRelease(self.object);
			}
			std::mem::forget(self);
		}
	}
}

impl Drop for IOObjectBaseRef
{
	fn drop(&mut self)
	{
		if self.object != io::IO_OBJECT_NULL
		{
			unsafe
			{
				io::IOObjectRelease(self.object);
			}
		}
	}
}

pub trait IOObject
{
	fn base_ref(&self) -> &IOObjectBaseRef;
	fn release(self);
}

pub trait IORegistryEntry : IOObject
{
	fn get_path(&self, plane : RegistryPlane) -> Result<String, io::IOReturn>
	{
		let mut path_buffer : Vec<libc::c_char> = vec![0; 512]; // io_string_t
		unsafe
		{
			let result = io::IORegistryEntryGetPath(self.base_ref().object, plane_cchar(plane), path_buffer.as_mut_ptr());
			if result == io::kIOReturnSuccess
			{
				let path_cstr = std::ffi::CStr::from_ptr(path_buffer.as_ptr());
				Result::Ok(path_cstr.to_str().unwrap().to_owned())
			}
			else
			{
				Result::Err(result)
			}
		}
	}
}

pub struct IORegistryEntryRef
{
	obj_ref : IOObjectBaseRef
}
impl IORegistryEntryRef
{
}

impl IOObject for IORegistryEntryRef
{
	fn base_ref(&self) -> &IOObjectBaseRef
	{
		&self.obj_ref
	}
	fn release(self)
	{
		self.obj_ref.release();
	}
}

impl IORegistryEntry for IORegistryEntryRef
{
}

pub struct IOServiceRef
{
	obj_ref : IOObjectBaseRef
}

impl IOServiceRef
{
	pub fn with_owned_service(service : io::io_service_t) -> IOServiceRef
	{
		IOServiceRef { obj_ref: IOObjectBaseRef { object: service } }
	}
}

impl IOObject for IOServiceRef
{
	fn base_ref(&self) -> &IOObjectBaseRef
	{
		&self.obj_ref
	}
	fn release(self)
	{
		self.obj_ref.release();
	}
}
impl IORegistryEntry for IOServiceRef
{
}

