pub fn array() -> String {
	String::from("\
import bird::memory::{self, BirdAlloc}
import bird::ptr::NULL;
import bird::operators::Index;

pub struct Array<T>
{
	ptr: *mut T,
	size: uint
}

impl Array<T>
{
	pub func new(size: uint) -> Result<Self, String>
	{
		var ptr = BirdAlloc::allocate(size_of<T>() * size)

		if (ptr == NULL)
		{
			return Err(\"Failed allocating memory\")
		}

		return Ok(Self { ptr, size })
	}

	pub func size(&self) -> uint
	{
		return self.size
	}

	pub func get(&self, index: uint) -> &T
	{
		return &*(self.ptr + index)
	}

	pub func get_mut(&mut self, index: uint) -> &mut T
	{
		return &mut *(self.ptr + index)
	}
}

impl Drop for Array<T>
{
	pub fn drop(&mut self)
	{
		BirdAlloc::free(self.ptr)
	}
}

impl Index for Array<T>
{
	type Output = T

	pub fn index(&self, index: uint) -> &Self::Output
	{
		return &*(self.ptr + index)
	}
}\
	")
}