/// Overflows input `f32` between **0.0** and **360.0**
/// 
/// * Returns: `degrees` between **0.0** and **360.0**
pub fn degrees_overflow( degrees:f32 ) -> f32 {
    let mut result = degrees;
    while result < 0.0 { result += 360.0; }
    
    if result > 360.0 { return result % 360.0; }
    else { return result; }
}