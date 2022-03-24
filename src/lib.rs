#[allow(dead_code)]

pub mod vector;

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