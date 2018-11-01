use ::std::cmp::PartialEq;
use ::std::fmt;
use ::std::ops::{Add, Div, Mul, Sub};

use super::{Bit, Num, OperationResult};


/// Binary: Sequence of Bits, ordered from least to most significant
pub struct Binary([Bit; 64]);

impl Binary {

    /// Construct a binary from a given Num
    pub fn from_int(n: Num) -> Binary {
        // Need 66 chars to represent 64bit, since it adds "0b" to beginning
        let bit_string = format!("{:#066b}", n);
        let mut binary = Binary::zero();

        bit_string.chars().skip(2)
            .map(|c| if c == '1' { Bit::On } else { Bit::Off })
            .enumerate()
            .for_each(|(i, bit)| binary.set(63 - i, bit));

        println!("\nx: {}\n{:#010b}\n{:?}", n, n, binary);
        binary
    }

    /// Parse own bits into a Num
    pub fn to_int(self) -> OperationResult {
        // Rust is a little inconsistent with how it handles negative binary numbers...
        //
        //   - Literals use negative, eg. -0b0000_0011 for -3)
        //   - format! outputs two's complement, eg. "0b1111_1101" for `format!("{:#010b}", -3)`
        //   - i8::from_str_radix expects a stringified version of a negative literal, rather than
        //      two's complement
        //
        // ... which means we can't simply format! -> build complement string w/ logic -> parse with
        // from_str_radix. If negative, need to convert from two's complement to negative literal

        // Capacity should fit full number and potentially "-" sign
        let mut s = String::with_capacity(65);
        let mut binary = self;

        if binary.is_negative() {
            // Take complement to make "positive" and then add sign
            s.push('-');
            binary = binary.complement();
        }

        // Run through bit array and push chars onto string, reversing order of bits
        for i in 0..64 {
            s.push(if binary.is_on_at(63 - i) { '1' } else { '0' });
        }

        Num::from_str_radix(s.as_str(), 2)
    }

	pub fn of(b: Bit) -> Binary {
		Binary([b; 64])
	}

	pub fn zero() -> Binary {
		Binary::of(Bit::Off)
	}

	pub fn one() -> Binary {
		let mut binary = Binary::zero();
		binary.set(0, Bit::On);
		binary
	}

	pub fn get(&self, i: usize) -> Bit {
		self.0[i]
	}

	pub fn set(&mut self, i: usize, b: Bit) {
		self.0[i] = b;
	}

	pub fn is_on_at(&self, i: usize) -> bool {
		match self.0[i] {
			Bit::On => true,
			_ => false,
		}
	}

	pub fn is_negative(&self) -> bool {
		self.is_on_at(63)
	}

    /// Inverts the sign of a Binary by flipping bits and adding 1
    pub fn complement(self) -> Binary {
        let mut comp = Binary::zero();

        for i in 0..64 {
            comp.set(i, !self.get(i));
        }

        comp + Binary::one()
    }
}

impl fmt::Debug for Binary {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0[..].fmt(formatter)
    }
}

impl PartialEq for Binary {
	fn eq(&self, other: &Binary) -> bool {
		for i in 0..63 {
			if self.get(i) != other.get(i) {
				return false;
			}
		}

		return true;
	}
}

impl Add for Binary {
    type Output = Binary;

    /// Basic implementation of full addition circuit
    fn add(self, other: Binary) -> Binary {
        let mut rval = Binary::zero();
        let mut carry = Bit::Off;

        for i in 0..64 {
            let result = match i {
                0 => Bit::half_adder(self.get(i), other.get(i)),
                _ => Bit::full_adder(self.get(i), other.get(i), carry),
            };

            rval.set(i, result.0);
            carry = result.1;
        }

        rval
    }
}

impl Sub for Binary {
    type Output = Binary;

    /// Simple subtractor that takes complement of other and adds to self
    fn sub(self, other: Binary) -> Binary {
        self + other.complement()
    }
}

impl Mul for Binary {
    type Output = Binary;

    /// Multiplier that uses basic series of shifts and adding partial products
    fn mul(self, other: Binary) -> Binary {
        let mut accumulator = Binary::zero();

        for i in 0..64 {
            let mut partial = Binary::zero();

            for j in 0..(64 - i) {
                partial.set(j + i, Bit::multiplier(self.get(j), other.get(i)));
            }

            accumulator = accumulator + partial;
        }

        accumulator
    }
}

impl Div for Binary {
    type Output = Binary;

    fn div(self, other: Binary) -> Binary {
        other
    }
}


#[cfg(test)]
mod tests {
    use super::{Bit, Binary};

    #[test]
    fn from_int_test() {
        let zero = Binary::zero();
        let one = Binary::one();
        let negative_one = Binary::of(Bit::On);

        let min = {
            let mut ba = Binary::of(Bit::Off);
            ba.set(63, Bit::On);
            ba
        };

        let max = {
            let mut ba = Binary::of(Bit::On);
            ba.set(63, Bit::Off);
            ba
        };

        assert_eq!(Binary::from_int(0), zero, "0");
        assert_eq!(Binary::from_int(1), one, "1");
        assert_eq!(Binary::from_int(-1), negative_one, "-1");
        assert_eq!(Binary::from_int(-9223372036854775808), min, "min");
        assert_eq!(Binary::from_int(9223372036854775807), max, "max");
    }

    #[test]
    fn to_int_test() {
        let zero = Binary::zero();
        let one = Binary::one();
        let negative_one = Binary::of(Bit::On);

        let min = {
            let mut ba = Binary::of(Bit::Off);
            ba.set(63, Bit::On);
            ba
        };

        let max = {
            let mut ba = Binary::of(Bit::On);
            ba.set(63, Bit::Off);
            ba
        };

        assert_eq!(Binary::to_int(zero), Ok(0), "0");
        assert_eq!(Binary::to_int(one), Ok(1), "1");
        assert_eq!(Binary::to_int(negative_one), Ok(-1), "-1");
        assert_eq!(Binary::to_int(min), Ok(::std::i64::MIN), "min");
        assert_eq!(Binary::to_int(max), Ok(::std::i64::MAX), "max");
    }
}