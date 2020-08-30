use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

#[wasm_bindgen]
pub fn rgb_to_hex(r: &str, g: &str, b: &str) -> String {
    let red: i32 = r.parse().unwrap();
    let green: i32 = g.parse().unwrap();
    let blue: i32 = b.parse().unwrap();
    
    let color = Color { red: red, green: green, blue: blue };
    
    let result = format!("#{:0>2x}{:0>2x}{:0>2x}", color.red, color.green, color.blue);
    println!("{}", result);

    return result;
}
