extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

#[wasm_bindgen]
pub async fn get_dominant_color(img: &HtmlImageElement) -> Option<String> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    /*let img = document.create_element("img").unwrap();
    let img: web_sys::HtmlImageElement = img
        .dyn_into::<web_sys::HtmlImageElement>()
        .map_err(|_| ())
        .unwrap();*/
    let load_promise = JsFuture::from(img.decode());
    load_promise.await.ok()?;
    let img_width = img.natural_width() as u32;
    let img_height = img.natural_height() as u32;
    canvas.set_width(img_width);
    canvas.set_height(img_height);
    context
        .draw_image_with_html_image_element(&img, 0.0, 0.0)
        .unwrap();
    let image_data = context
        .get_image_data(0.0, 0.0, img_width as f64, img_height as f64)
        .unwrap()
        .data();
    let mut color_counts = std::collections::HashMap::new();
    let mut max_count = 0;
    let mut dominant_color = String::new();
    for i in (0..image_data.len()).step_by(4) {
        if (image_data[i + 3] as u32) == 0 {
            continue;
        }

        let rgba = round_color(
            image_data[i] as u32,
            image_data[i + 1] as u32,
            image_data[i + 2] as u32,
        );
        let count = color_counts.entry(rgba.clone()).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            dominant_color = rgba;
        }
    }
    Some(format!("rgba({})", dominant_color))
}

pub fn round_color(r: u32, g: u32, b: u32) -> String {
    let factor = 10;
    let step = 255 / factor;
    let r = (r / step) * step;
    let g = (g / step) * step;
    let b = (b / step) * step;
    format!("rgba({},{},{},1)", r, g, b)
}
