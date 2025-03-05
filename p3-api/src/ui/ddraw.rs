use std::mem;

pub fn ddraw_sgl_draw_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let function: extern "cdecl" fn(x1: f32, y1: f32, x2: f32, y2: f32) = unsafe { mem::transmute(0x004BB390) };
    function(x1, y1, x2, y2)
}
