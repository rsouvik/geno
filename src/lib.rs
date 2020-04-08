#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//pub mod geno {

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(id1: i32, id2: i32, id3: i32) {
    id1 + id2 + id3;
}

#[wasm_bindgen]
pub fn randnum() {
    rand::random::<i32>();
}

#[wasm_bindgen]
pub fn hel() {
    //print!(" Hello");
    alert(" Hello");
}

//}
