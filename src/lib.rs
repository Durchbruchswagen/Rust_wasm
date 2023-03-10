mod utils;
mod frac_util;

use wasm_bindgen::prelude::*;
//use wasm_bindgen_test::__rt::log;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello,c ccwasm-game-of-life!");
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    red: Vec<u8>,
    green: Vec<u8>,
    blue: Vec<u8>
}
#[wasm_bindgen]
impl Universe {

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn red(&self) -> *const u8 {
        self.red.as_ptr()
    }
    pub fn green(&self) -> *const u8 {
        self.green.as_ptr()
    }
    pub fn blue(&self) -> *const u8 {
        self.blue.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    pub fn tick(&mut self,real:f32,im:f32) {
        for y in 0..self.height{
            for x in 0..self.width{
                let rest=frac_util::computecolor(x,y,self.height,self.width,real,im);
                let ind=self.get_index(y, x);
                self.red[ind]=rest.0 as u8;
                self.green[ind]=rest.1 as u8;
                self.blue[ind]=rest.2 as u8;
            }
        }     
    }
    // ...
    pub fn new() -> Universe {
        let width:u32 = 1024;
        let height:u32 = 1024;
        let red:Vec<u8>=vec![0;(width*height) as usize];
        let green:Vec<u8>=vec![0;(width*height) as usize];
        let blue:Vec<u8>=vec![0;(width*height) as usize];

        Universe {
            width,
            height,
            red,
            green,
            blue
        }
    }
}
