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

use std::collections::HashMap;
use std::io::{Result, Read, Write, Error, ErrorKind};

use backend::Backend;
use hash::ContentHasher;

impl<H> Backend<H> for HashMap<H::Digest, Vec<u8>>
	where H: ContentHasher,
{
	fn store(
		&mut self,
		source: &Fn(&mut Write, &mut Backend<H>) -> Result<H::Digest>,
	) -> Result<H::Digest> {
		let mut vec = vec![];
		let hash = try!(source(&mut vec, self));
		self.insert(hash.clone(), vec);
		Ok(hash)
	}
	fn request(
		&self,
		hash: &H::Digest,
		read: &Fn(&mut Read) -> Result<()>,
	) -> Result<()> {
		let vec = try!(self.get(hash)
					   .ok_or(Error::new(ErrorKind::NotFound, "")));
		read(&mut &vec[..])
	}
}
