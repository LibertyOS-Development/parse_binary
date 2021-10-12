#![no_std]
use core::mem;
use core::slice::from_raw_parts;
use core::str::{from_utf8, from_utf8_unchecked};

pub fn read<T: Pod>(input: &[u8]) -> &T
{
	assert!(mem::size_of::<T>() <= input.len());
	let address = input.as_ptr() as usize;
	assert!((address & (mem::align_of::<T>() - 1)) == 0);
	unsafe
	{
		readunsafe(input)
	}
}

pub fn readarray<T: Pod>(input: &[u8]) -> &[T]
{
	let tsize = mem::size_of::<T>();
	assert!(tsize > 0, "[ERR] CANNOT READ ARRAYS OF ZERO-SIZED TYPES");
	assert!(input.len() % tsize == 0);
	let address = input.as_ptr() as usize;
	assert!(address & (mem::align_of::<T>() - 1) == 0);
	unsafe
	{
		read_arrayunsafe(input)
	}
}

pub fn readstr(input: &[u8]) -> &str
{
	from_utf8(read_strbytes(input)).expect("[ERR] INVALID UTF-8 STRING")
}

pub fn readstr2null(input: &[u8]) -> StrReadIter
{
	StrReadIter
	{
		dat: input
	}
}

pub unsafe trait Pod: Sized {}

unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for u128 {}
unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for i128 {}

pub unsafe fn readunsafe<T: Sized>(input: &[u8]) -> &T
{
	&*(input.as_ptr() as *const T)
}

pub unsafe fn read_arrayunsafe<T: Sized>(input: &[u8]) -> &[T]
{
	let ptr = input.as_ptr() as *const T;
	from_raw_parts(ptr, input.len() / mem::size_of::<T>())
}

pub unsafe fn read_strunsafe(input: &[u8]) -> &str
{
	from_utf8_unchecked(read_strbytes(input))
}

#[derive(Clone, Debug)]
pub struct StrReadIter<'a>
{
	dat: &'a [u8],
}

impl<'a> Iterator for StrReadIter<'a>
{
	type Item = &'a str;
	fn next(&mut self) -> Option<&'a str>
	{
		if self.dat.is_empty() || self.dat[0] == 0
		{
			return None;
		}
		let result = readstr(self.dat);
		self.dat = &self.dat[result.len() + 1..];
		Some(result)
	}
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		(0, Some(self.dat.len() / 2))
	}
}

fn read_strbytes(input: &[u8]) -> &[u8]
{
	for (i, byte) in input.iter().enumerate()
	{
		if *byte == 0
		{
			return &input[..i];
		}
	}
	panic!("[ERR] NO NULL BYTE IN INPUT");
}
