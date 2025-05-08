use wasm_bindgen::prelude::*;
use hello_world_lib;

#[wasm_bindgen]
pub fn hello_world() -> String {
    hello_world_lib::hello_world().to_string()
}

#[wasm_bindgen]
pub struct Calculator {
    calc: hello_world_lib::Calc,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new(value: f64) -> Self {
        Calculator {
            calc: hello_world_lib::Calc::new(value),
        }
    }

    pub fn sum(&self, other: f64) -> f64 {
        self.calc.sum(other)
    }

    pub fn subtract(&self, other: f64) -> f64 {
        self.calc.subtract(other)
    }

    pub fn multiply(&self, other: f64) -> f64 {
        self.calc.multiply(other)
    }

    pub fn divide(&self, other: f64) -> Option<f64> {
        self.calc.divide(other)
    }
} 