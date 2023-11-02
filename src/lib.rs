use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    fn alert(message: &str);
}

#[wasm_bindgen]
pub fn start() {
    alert("Hello World!")
}