pub mod hexadecimal;
pub mod angles;

use super::PI;

/// Returns: the **largest** value in the given `array`
pub fn max<N>( values:&[N] ) -> N
where N:PartialOrd + Copy
{
    let mut largest = values[0];
    let mut i = 0;
    while i < values.len() {
        if values[i] > largest {
            largest = values[i];
        }
        i += 1;
    }

    return largest;
}

/// Returns: the **smallest** value in the given `array`
pub fn min<N>( values:&[N] ) -> N
where N:PartialOrd + Copy
{
    let mut smallest = values[0];
    let mut i = 0;
    while i < values.len() {
        if values[i] < smallest {
            smallest = values[i];
        }
        i += 1;
    }

    return smallest;
}

/// Clamps the given `value` between given `minimum` and `maximum`.
/// 
/// * Returns: `mininimum` if `value` is *less* than `minimum`.
/// 
/// * Returns: `maximum` if `value` is *greater* than `maximum`.
/// 
/// * Returns: `value` if `value` is between the `minimum` and `maximum` range.
/// 
pub fn clamp<N>( value:N, min:N, max:N ) -> N
where N:PartialOrd
{
    assert!( min <= max );
    if value < min {
        return min;
    } else if value > max {
        return max;
    }

    return value;
}

/// Adds `lhs` and `rhs` with *overflow* check
/// 
/// * Returns: **255** if `result` overflows
/// 
/// * Returns: `result` if no overflow occurs
pub fn u8_add_overflow_max_clamp( lhs:u8, rhs:u8 ) -> u8 {
    let ( result, is_overflowing ) = u8::overflowing_add(lhs, rhs);
    if is_overflowing {
        return u8::MAX;
    } else {
        return result;
    }
}

/// Subtracts `rhs` from `lhs` with *overflow* check
/// 
/// * Returns: **0** if `result` overflows
/// 
/// * Returns: `result` if no overflow occurs
pub fn u8_sub_overflow_min_clamp( lhs:u8, rhs:u8 ) -> u8 {
    let ( result, is_overflowing ) = u8::overflowing_sub(lhs, rhs);
    if is_overflowing {
        return 0_u8;
    } else {
        return result;
    }
}

/// Multiplies `lhs` and `rhs` with *overflow* check
/// 
/// * Returns: **255** if `result` overflows
/// 
/// * Returns: `result` if no overflow occurs
pub fn u8_mul_overflow_max_clamp( lhs:u8, rhs:u8 ) -> u8 {
    let ( result, is_overflowing ) = u8::overflowing_mul(lhs, rhs);
    if is_overflowing {
        return u8::MAX;
    } else {
        return result;
    }
}