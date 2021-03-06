use core::fmt::Write;

/// Decodes input `str` into bytes `Vec`
/// 
/// * Returns: `Vec` of bytes if input `str` has even number length
/// 
/// * Returns: `Error` as `String` if input `str` has odd number length
pub fn decode_hex_str( hex:&str ) -> Result<Vec<u8>, String> {

    if hex.len() % 2 != 0 {
        return Err(
            format!("DECODE HEXADECIMAL ERROR: Input hex string should have even number length!")
        );
    }

    let mut result:Vec<u8> = Vec::with_capacity( hex.len() / 2 );
    let mut i = 0;
    while i < hex.len() {

        let byte = match u8::from_str_radix( &hex[ i..i + 2 ], 16 ) {
            Ok(result) => result,
            Err(error) => return Err( format!("{}", error) ),
        };

        result.push(byte);

        i += 2;
    }

    return Ok( result );

}

/// Decodes input `str` into `u8;3`
/// 
/// * Returns: `u8;3` if input `str` is formatted properly
/// 
/// * Returns: `Error` as `String` if input `str` is not formatted properly
pub fn decode_hex_rgb( hex:&str ) -> Result<[u8;3], String> {

    let mut hexadecimal = String::from(hex);

    if hexadecimal.contains('#') {
        hexadecimal = String::from(hexadecimal.trim_start_matches('#'));
    }

    if hexadecimal.len() != 6 {
        return Err(
            format!("DECODE HEXADECIMAL ERROR: Input hex string formatted incorrectly!")
        );
    }

    let mut result:[u8;3] = [0, 0, 0];
    let mut i = 0;
    let mut j = 0;
    while i < hexadecimal.len() {

        let byte = match u8::from_str_radix( &hexadecimal[ i..i + 2 ], 16 ) {
            Ok(result) => result,
            Err(error) => return Err( format!("{}", error) ),
        };

        result[j] = byte;

        i += 2;
        j += 1;
    }

    return Ok( result );

}

/// Encodes input bytes `array` into *hexadecimal* `String`
pub fn encode_hex( bytes:&[u8] ) -> String {
    let mut result = String::with_capacity( bytes.len() * 2 );

    for byte in bytes {
        write!( &mut result, "{:02x}", byte ).unwrap();
    }

    return result;
}