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

extern crate blake2_rfc;
extern crate parking_lot;
extern crate byteorder;
extern crate rand;

#[cfg(test)]
extern crate tempdir;
#[cfg(test)]
mod test_common;

mod default;
mod content;
mod store;
mod backend;
mod hash;
mod std_impls;

pub use store::Store;
pub use content::{Content, Sink, Source};
pub use hash::ContentHasher;
