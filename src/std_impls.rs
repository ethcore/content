// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::io::{Read, Write, Result, Error, ErrorKind};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use content::{Content, Source, Sink};
use hash::ContentHasher;

/// Content implementations for common std things

impl<T, H> Content<H> for Option<T> where T: Content<H>, H: ContentHasher {
	fn to_content(&self, sink: &mut Sink<H>) -> Result<()> {
		match *self {
			Some(ref t) => {
				try!(sink.write(&[1]));
				t.to_content(sink)
			},
			None => sink.write_all(&[0])
		}
	}
	fn from_content(source: &mut Source<H>) -> Result<Self> {
		let mut byte = [0];
		try!(source.read_exact(&mut byte));
		match byte[0] {
			0 => Ok(None),
			1 => Ok(Some(try!(T::from_content(source)))),
			_ => Err(Error::new(
				ErrorKind::Other, "Invalid Option<T> encoding!")
			),
		}
	}
}

// Box just needs to pass through the writers and readers
// of T, wrapping the read value in a new Box.
//
// This means that Box<T> and T have an identical content
// hash, differing only on the type level!
impl<T, H> Content<H> for Box<T> where T: Content<H>, H: ContentHasher {
	fn to_content(&self, sink: &mut Sink<H>) -> Result<()> {
		(self as &T).to_content(sink)
	}
	fn from_content(source: &mut Source<H>) -> Result<Self> {
		Ok(Box::new(try!(T::from_content(source))))
	}
}

impl<H> Content<H> for u8 where H: ContentHasher {
	fn to_content(&self, sink: &mut Sink<H>) -> Result<()> {
		sink.write_all(&[*self])
	}
	fn from_content(source: &mut Source<H>) -> Result<Self> {
		let b = &mut [0u8];
		try!(source.read_exact(b));
		Ok(b[0])
	}
}

impl<H> Content<H> for () where H: ContentHasher {
	fn to_content(&self, _: &mut Sink<H>) -> Result<()> {
		Ok(())
	}
	fn from_content(_: &mut Source<H>) -> Result<Self> {
		Ok(())
	}
}

macro_rules! number {
	( $t:ty: $read:ident, $write:ident ) => {
		impl<H> Content<H> for $t where H: ContentHasher {
			fn to_content(&self, sink: &mut Sink<H>) -> Result<()> {
				sink.$write::<BigEndian>(*self)
			}
			fn from_content(source: &mut Source<H>) -> Result<Self> {
				source.$read::<BigEndian>()
			}
		}
	}
}

number!(u64: read_u64, write_u64);
number!(u32: read_u32, write_u32);
number!(u16: read_u16, write_u16);

number!(i64: read_i64, write_i64);
number!(i32: read_i32, write_i32);
number!(i16: read_i16, write_i16);

#[cfg(test)]
mod tests {
	use store::Store;
	use content::Content;
	use std::fmt::Debug;
	use default::BlakeWrap;

	fn put_get<T: Content<BlakeWrap> + Debug + PartialEq>(t: T) {
		let mut store: Store<_, BlakeWrap> = Store::new();
		let hash = store.put(&t).unwrap();
		assert_eq!(store.get(&hash).unwrap(), t);
	}

	#[test]
	fn std() {
		put_get(Some(46u64));
		put_get(None as Option<u64>);
		put_get(38u64);
	}

}
