use super::PI;

/// Overflows input `f32` between **0.0** and **360.0**
/// 
/// * Returns: `degrees` between **0.0** and **360.0**
pub fn degrees_overflow( degrees:f32 ) -> f32 {
    let mut result = degrees;

    if result < 0.0 {
        result += 360.0;
    }
    
    if result > 360.0 {
        return result % 360.0;
    } else {
        return result;
    }
}

/// Returns: radians as`f32`
pub fn degrees_to_radians(d:f32) -> f32 {
    d * PI/180.0
}

/// Returns: degrees as `f32`
pub fn radians_to_degrees(r:f32) -> f32 {
    r * 180.0/PI
}