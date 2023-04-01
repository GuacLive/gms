use super::errors::H264Error;
use bytesio::bits_reader::BitsReader;

// ue(v) in 9.1 Parsing process for Exp-Golomb codes
// ISO_IEC_14496-10-AVC-2012.pdf, page 227.
// Syntax elements coded as ue(v), me(v), or se(v) are Exp-Golomb-coded.
//      leadingZeroBits = -1;
//      for( b = 0; !b; leadingZeroBits++ )
//          b = read_bits( 1 )
// The variable codeNum is then assigned as follows:
//      codeNum = (2<<leadingZeroBits) - 1 + read_bits( leadingZeroBits )
pub fn read_uev(bit_reader: &mut BitsReader) -> Result<u32, H264Error> {
    let mut leading_zeros_bits: usize = 0;

    loop {
        if bit_reader.read_bit()? != 0 {
            break;
        }
        leading_zeros_bits += 1;
    }
    let code_num = (1 << leading_zeros_bits) - 1 + bit_reader.read_n_bits(leading_zeros_bits)?;
    Ok(code_num as u32)
}

// ISO_IEC_14496-10-AVC-2012.pdf, page 229.
pub fn read_sev(bit_reader: &mut BitsReader) -> Result<i32, H264Error> {
    let code_num = read_uev(bit_reader)?;

    let negative: i64 = if code_num % 2 == 0 { -1 } else { 1 };
    let se_value = (code_num as i64 + 1) / 2 * negative;
    Ok(se_value as i32)
}

#[cfg(test)]
mod tests {

    use super::read_uev;
    use bytes::BytesMut;
    use bytesio::bits_reader::BitsReader;
    use bytesio::bytes_reader::BytesReader;

    #[test]
    fn test_read_uev() {
        // 0 => 1 => 1
        // 1 => 10 => 010
        // 2 => 11 => 011
        // 3 => 100 => 00100
        // 4 => 101 => 00101
        // 5 => 110 => 00110
        // 6 => 111 => 00111
        // 7 => 1000 => 0001000
        // 8 => 1001 => 0001001

        let mut bytes_reader = BytesReader::new(BytesMut::new());
        bytes_reader.extend_from_slice(&[0b00000001]);
        bytes_reader.extend_from_slice(&[0b00000010]);
        bytes_reader.extend_from_slice(&[0b00000011]);
        bytes_reader.extend_from_slice(&[0b00000100]);
        bytes_reader.extend_from_slice(&[0b00000101]);
        bytes_reader.extend_from_slice(&[0b00000110]);
        bytes_reader.extend_from_slice(&[0b00000111]);
        bytes_reader.extend_from_slice(&[0b00001000]);
        bytes_reader.extend_from_slice(&[0b00001001]);

        let mut bits_reader = BitsReader::new(bytes_reader);

        bits_reader.read_n_bits(7).unwrap();
        let v1 = read_uev(&mut bits_reader).unwrap();
        println!("=={v1}==");
        assert!(v1 == 0);

        bits_reader.read_n_bits(5).unwrap();
        let v2 = read_uev(&mut bits_reader).unwrap();
        println!("=={v2}==");
        assert!(v2 == 1);

        bits_reader.read_n_bits(5).unwrap();
        let v3 = read_uev(&mut bits_reader).unwrap();
        println!("=={v3}==");
        assert!(v3 == 2);

        bits_reader.read_n_bits(3).unwrap();
        let v4 = read_uev(&mut bits_reader).unwrap();
        println!("=={v4}==");
        assert!(v4 == 3);

        bits_reader.read_n_bits(3).unwrap();
        let v5 = read_uev(&mut bits_reader).unwrap();
        println!("=={v5}==");
        assert!(v5 == 4);

        bits_reader.read_n_bits(3).unwrap();
        let v6 = read_uev(&mut bits_reader).unwrap();
        println!("=={v6}==");
        assert!(v6 == 5);

        bits_reader.read_n_bits(3).unwrap();
        let v7 = read_uev(&mut bits_reader).unwrap();
        println!("=={v7}==");
        assert!(v7 == 6);

        bits_reader.read_n_bits(1).unwrap();
        let v8 = read_uev(&mut bits_reader).unwrap();
        println!("=={v8}==");
        assert!(v8 == 7);

        bits_reader.read_n_bits(1).unwrap();
        let v9 = read_uev(&mut bits_reader).unwrap();
        println!("=={v9}==");
        assert!(v9 == 8);
    }
}
