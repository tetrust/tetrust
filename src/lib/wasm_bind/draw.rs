use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn draw_block(
    context: CanvasRenderingContext2d,
    x: f64,
    y: f64,
    block_width_size: f64,
    block_height_size: f64,
    color: &str,
) {
    let stroke_size = 0.5;

    context.set_stroke_style(&JsValue::from_str(color)); // 테두리 색상(불투명도 조절 어떻게하나요?)
    context.set_fill_style(&JsValue::from_str(color)); // 내부 색상
    context.fill_rect(
        x,
        y,
        block_width_size - stroke_size,
        block_height_size - stroke_size,
    );
    context.stroke_rect(
        x,
        y,
        block_width_size - stroke_size,
        block_height_size - stroke_size,
    );
}
