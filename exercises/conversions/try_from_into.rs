// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}



// Your task is to complete this implementation and return an Ok result of inner
// type Color. You need to create an implementation for a tuple of three
// integers, an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile
// time, but the slice implementation needs to check the slice length! Also note
// that correct RGB color values must be integers in the 0..=255 range.

// Tuple implementation


// 定义解析错误类型


// 实现 TryFrom for tuple
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (r, g, b) = tuple;
        if (0..=255).contains(&r) && (0..=255).contains(&g) && (0..=255).contains(&b) {
            Ok(Color { red: r as u8, green: g as u8, blue: b as u8 })
        } else {
            Err(IntoColorError::IntConversion)
        }
    }
}

// 实现 TryFrom for array
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Self::try_from((arr[0], arr[1], arr[2])) // 复用 tuple 实现
    }
}

// 实现 TryFrom for slice
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        Self::try_from((slice[0], slice[1], slice[2])) // 复用 tuple 实现
    }
}

fn main() {
    // 测试成功的转换
    let c1: Color = (255, 128, 64).try_into().unwrap();
    println!("{:?}", c1); // 输出: Color { red: 255, green: 128, blue: 64 }

    let c2: Color = [200, 150, 100].try_into().unwrap();
    println!("{:?}", c2); // 输出: Color { red: 200, green: 150, blue: 100 }

    let c3: Color = (&[0, 128, 255][..]).try_into().unwrap();
    println!("{:?}", c3); // 输出: Color { red: 0, green: 128, blue: 255 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_valid() {
        assert_eq!(Color::try_from((100, 150, 200)), Ok(Color { red: 100, green: 150, blue: 200 }));
    }

    #[test]
    fn test_tuple_out_of_range() {
        assert_eq!(Color::try_from((-1, 255, 256)), Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_valid() {
        assert_eq!(Color::try_from([50, 100, 150]), Ok(Color { red: 50, green: 100, blue: 150 }));
    }

    #[test]
    fn test_array_out_of_range() {
        assert_eq!(Color::try_from([0, 500, 255]), Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_slice_valid() {
        let arr = [30, 60, 90];
        assert_eq!(Color::try_from(&arr[..]), Ok(Color { red: 30, green: 60, blue: 90 }));
    }

    #[test]
    fn test_slice_bad_length() {
        let arr = [50, 100]; // 长度不足
        assert_eq!(Color::try_from(&arr[..]), Err(IntoColorError::BadLen));
    }

    #[test]
    fn test_slice_out_of_range() {
        let arr = [10, 20, 300]; // 300 超出范围
        assert_eq!(Color::try_from(&arr[..]), Err(IntoColorError::IntConversion));
    }
}
