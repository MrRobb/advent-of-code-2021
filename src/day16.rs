#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

use bitvec::prelude::*;
use itertools::Itertools;

struct BitBufReader {
	bits: BitVec<Msb0, u8>,
	position: usize,
}

impl BitBufReader {
	fn new(bits: BitVec<Msb0, u8>) -> Self {
		Self { bits, position: 0 }
	}

	fn read(&mut self, n: usize) -> &BitSlice<Msb0, u8> {
		let slice = &self.bits[self.position..self.position + n];
		self.position += n;
		slice
	}

	fn is_end(&self) -> bool {
		self.position >= self.bits.len()
	}
}

#[derive(Debug)]
struct Header {
	version: u8,
	type_id: u8,
}

impl Header {
	fn parse(bytes: &mut BitBufReader) -> Self {
		Self {
			version: bytes.read(3).load_be(),
			type_id: bytes.read(3).load_be(),
		}
	}
}

#[derive(Debug)]
enum Operator {
	Sum,
	Product,
	Minimum,
	Maximum,
	GreaterThan,
	LessThan,
	EqualTo,
}

#[derive(Debug)]
enum Body {
	Literal(u128),
	Operator(Vec<Packet>, Operator),
}

impl Body {
	pub fn parse(header: &Header, bytes: &mut BitBufReader) -> Self {
		match header.type_id {
			0 => Self::parse_operator(bytes, Operator::Sum),
			1 => Self::parse_operator(bytes, Operator::Product),
			2 => Self::parse_operator(bytes, Operator::Minimum),
			3 => Self::parse_operator(bytes, Operator::Maximum),
			4 => Self::parse_literal(bytes),
			5 => Self::parse_operator(bytes, Operator::GreaterThan),
			6 => Self::parse_operator(bytes, Operator::LessThan),
			7 => Self::parse_operator(bytes, Operator::EqualTo),
			_ => unreachable!(),
		}
	}

	fn parse_literal(bytes: &mut BitBufReader) -> Self {
		let mut bv = BitVec::<Msb0, u8>::new();
		loop {
			let bits = bytes.read(5);
			let b = bits.get(1..).unwrap();
			bv.extend(b);
			if !*bits.get(0).unwrap() {
				break;
			}
		}
		Self::Literal(bv.load_be())
	}

	fn parse_operator(bytes: &mut BitBufReader, operator: Operator) -> Self {
		if *bytes.read(1).get(0).unwrap() {
			let n_packets: usize = bytes.read(11).load_be();
			Self::Operator(Self::parse_n_packets(bytes, n_packets), operator)
		}
		else {
			let n_bits: usize = bytes.read(15).load_be();
			Self::Operator(Self::parse_n_bits(bytes, n_bits), operator)
		}
	}

	fn parse_n_bits(bytes: &mut BitBufReader, n_bits: usize) -> Vec<Packet> {
		let bits = bytes.read(n_bits);
		let mut buff = BitBufReader::new(bits.to_bitvec());
		(0..)
			.map_while(|_| {
				if buff.is_end() {
					None
				}
				else {
					Some(Packet::parse(&mut buff))
				}
			})
			.collect()
	}

	fn parse_n_packets(bytes: &mut BitBufReader, n_packets: usize) -> Vec<Packet> {
		(0..n_packets).map(|_| Packet::parse(bytes)).collect()
	}

	fn eval(self) -> u128 {
		match self {
			Body::Literal(x) => x,
			Body::Operator(v, Operator::Sum) => v.into_iter().map(Packet::eval).sum(),
			Body::Operator(v, Operator::Product) => v.into_iter().map(Packet::eval).product(),
			Body::Operator(v, Operator::Minimum) => v.into_iter().map(Packet::eval).min().unwrap(),
			Body::Operator(v, Operator::Maximum) => v.into_iter().map(Packet::eval).max().unwrap(),
			Body::Operator(v, Operator::GreaterThan) => {
				v.into_iter().tuples().any(|(p0, p1)| p0.eval() > p1.eval()).into()
			},
			Body::Operator(v, Operator::LessThan) => {
				v.into_iter().tuples().any(|(p0, p1)| p0.eval() < p1.eval()).into()
			},
			Body::Operator(v, Operator::EqualTo) => {
				v.into_iter().tuples().any(|(p0, p1)| p0.eval() == p1.eval()).into()
			},
		}
	}

	fn count_versions(&self) -> u64 {
		match self {
			Body::Literal(_) => 0,
			Body::Operator(ps, _) => ps.iter().map(Packet::count_versions).sum(),
		}
	}
}

#[derive(Debug)]
struct Packet {
	header: Header,
	body: Body,
}

impl Packet {
	fn parse(bytes: &mut BitBufReader) -> Self {
		let header = Header::parse(bytes);
		let body = Body::parse(&header, bytes);
		Self { header, body }
	}

	fn count_versions(&self) -> u64 {
		u64::from(self.header.version) + self.body.count_versions()
	}

	fn eval(self) -> u128 {
		self.body.eval()
	}
}

pub fn calculate_sum_versions(input: &str) -> u64 {
	let bin = hex::decode(input).unwrap();
	let bits = BitVec::from_slice(&bin).unwrap();
	let mut buff = BitBufReader::new(bits);
	let packet = Packet::parse(&mut buff);
	packet.count_versions()
}

pub fn calculate_eval_expression(input: &str) -> u128 {
	let bin = hex::decode(input).unwrap();
	let bits = BitVec::from_slice(&bin).unwrap();
	let mut buff = BitBufReader::new(bits);
	let packet = Packet::parse(&mut buff);
	packet.eval()
}

pub fn main() {
	let input = read_to_string("input/day16/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_sum_versions(&input));
	println!("PART 2 = {}", calculate_eval_expression(&input));
	println!("Execution time: {:?}", now.elapsed());
}
