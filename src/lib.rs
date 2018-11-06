pub mod types;

use types::{Binary, ConversionResult};

pub fn add(x: i64, y: i64) -> ConversionResult {
    execute(|a, b| a + b, x, y)
}

pub fn subtract(x: i64, y: i64) -> ConversionResult {
    execute(|a, b| a - b, x, y)
}

pub fn multiply(x: i64, y: i64) -> ConversionResult {
    execute(|a, b| a * b, x, y)
}

pub fn divide(x: i64, y: i64) ->  ConversionResult {
    execute(|a, b| a / b, x, y)
}

fn execute(f: fn(&Binary, &Binary) -> Binary, x: i64, y: i64) -> ConversionResult {
    let bx = Binary::from_int(x);
    let by = Binary::from_int(y);
    let result = f(&bx, &by);

    println!("{:?}\n{}\n", bx, x);
    println!("{:?}\n{}\n", by, y);
    println!("{:?}", result);

    result.to_int()
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_test() {
        let args = [
            (0, 0), (0, 1), (0, 100), (0, 500),
            (1, 2), (1, 200), (1, 200000),
            (123321, 192392), (98498239, 1238723),
            (0, ::std::i64::MAX),

            (-1, 0), (0, -1), (0, -100), (0, -500),
            (1, -2), (1, -200), (1, -200000),
            (-1, -2), (-1, -200), (-1, -200000),
            (-123321, -192392), (-98498239, 1238723),
            (0, ::std::i64::MIN), (1, ::std::i64::MIN),

            (123, -32), (583, -91283), (-912389, 49823),
            (::std::i64::MIN, ::std::i64::MAX)
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::add(*x, *y), Ok(x + y), "{} + {}", x, y);
        }
    }

    #[test]
    fn subtract_test() {
        let args = [
            (0, 1), (0, 100), (0, 500),
            (1, 2), (1, 200), (1, 200000),
            (123321, 192392), (98498239, 1238723),
            (1, ::std::i64::MAX),

            (0, -1), (0, -100), (0, -500),
            (1, -2), (1, -200), (1, -200000),
            (-1, -2), (-1, -200), (-1, -200000),
            (-123321, -192392), (-98498239, 1238723),
            (-1, ::std::i64::MIN), (2, ::std::i64::MAX),

            (123, -32), (583, -91283), (-912389, 49823)
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::subtract(*x, *y), Ok(x - y), "{} - {}", x, y);
        }
    }

    #[test]
    fn multiply_test() {
        let args = [
            (0, 0), (0, 1), (1, 0),
            (0, -1), (-1, 0), (1, -1), (3, 5),
            (1023, 983298), (9183032, 9283983),
            (-51923, -938983), (-98329838, -29389238),
            (29389, -9238982), (-9238983, 928392838)
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::multiply(*x, *y), Ok(x * y), "{} * {}", x, y);
        }
    }
}
