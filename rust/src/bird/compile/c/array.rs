pub fn array() -> String {
	String::from("\
import bird::memory::{self, BirdAlloc}
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

		if (ptr == null)
		{
			return Err(\"Failed allocating memory\")
		}

		return Ok(Self { ptr, size })
	}

	pub func size(&self) -> uint
	{
		return self.size
	}

	pub func get(&self, index: uint) -> Option<&T>
	{
		if (index >= self.size)
		{
			return None
		}

		return Some(&*(self.ptr + index))
	}

	pub func get_mut(&mut self, index: uint) -> Option<&mut T>
	{
		if (index >= self.size)
		{
			return None
		}

		return &mut *(self.ptr + index)
	}
}

impl Drop for Array<T>
{
	pub func drop(&mut self)
	{
		BirdAlloc::free(self.ptr)
	}
}

impl Index for Array<T>
{
	type Output = T

	pub func index(&self, index: uint) -> &Output
	{
		if (index >= self.size)
		{
			panic(\"Index out of bounds\")
		}

		return &*(self.ptr + index)
	}
}\
	")
}