extern crate texture2d_decoder_sys as ffi;

pub fn decode_pvrtc(data: &[u8], w: i32, h: i32, image: &mut [u8], is2bpp: bool) {
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodePVRTC(input, w, h, output, if is2bpp { 1 } else { 0 }) };
}

pub fn decode_etc1(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 8) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeETC1(input, w, h, output) };
}

pub fn decode_etc2(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 8) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeETC2(input, w, h, output) };
}

pub fn decode_etc2a1(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 8) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeETC2A1(input, w, h, output) };
}

pub fn decode_etc2a8(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 16) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeETC2A8(input, w, h, output) };
}

pub fn decode_eacr(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 8) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeEACR(input, w, h, output) };
}

pub fn decode_eacr_signed(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 8) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeEACRSigned(input, w, h, output) };
}

pub fn decode_eacrg(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 16) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeEACRG(input, w, h, output) };
}

pub fn decode_eacrg_signed(data: &[u8], w: i32, h: i32, image: &mut [u8]) {
    let num_blocks_x = (w + 3) / 4;
    let num_blocks_y = (h + 3) / 4;

    assert!(data.len() >= (num_blocks_x * num_blocks_y * 16) as usize);
    assert!(image.len() >= (w * h * 4) as usize);

    let input = data.as_ptr() as *mut _;
    let output = image.as_mut_ptr() as *mut _;

    unsafe { ffi::DecodeEACRGSigned(input, w, h, output) };
}
