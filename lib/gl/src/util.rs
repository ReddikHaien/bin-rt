use std::convert::TryInto;

pub fn slice_to_int(slice: &[u8], offset: usize) -> u32{
    if slice.len() - offset >= 4{
        let bytes: [u8;4] = slice[offset..offset+4].try_into().unwrap();
        u32::from_be_bytes(bytes)
    }
    else{
        0u32
    }
}

pub fn slice_to_float(slice: &[u8], offset: usize) -> f32{
    if slice.len()-offset >= 4{
        let bytes: [u8;4] = slice[offset..offset+4].try_into().unwrap();
        f32::from_be_bytes(bytes)
    }
    else{
        0f32
    }
}