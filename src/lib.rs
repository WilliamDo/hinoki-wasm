mod utils;

use std::f64;
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn drawSomething() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("tournament-tree-canvas").unwrap();
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

    draw_tree(context, 4);

}

fn draw_tree(context: web_sys::CanvasRenderingContext2d, levels: u32) {

    let player_box_height = 40.0;
    let player_box_width = 120.0;
    let player_box_gap = 20.0;

    for i in 0..levels {
        let number_nodes = 2u32.pow(levels - i - 1);
        let offset = 2u32.pow(i) as f64 * (player_box_height + player_box_gap);
        let horizontal_bias = (i + 1) as f64 * player_box_gap + i as f64 * player_box_width;

        for node_idx in 0..number_nodes {                    
            let vertical_bias = ((2u32.pow(i) - 1) as f64 * (player_box_gap + player_box_height)) / 2.0;
            context.stroke_rect(
                horizontal_bias,
                node_idx as f64 * offset + vertical_bias,
                player_box_width,
                player_box_height
            );
        }

    }

}

pub enum TournamentTree {
    Empty,
    Node { left: Box<TournamentTree>, right: Box<TournamentTree> },
}

pub fn depth_of_tree(root: &TournamentTree) -> u32 {
    match root {
        TournamentTree::Empty => 0,
        TournamentTree::Node { left, right } => {
            let depth_left = depth_of_tree(&*left);
            let depth_right = depth_of_tree(&*right);
            1 + cmp::max(depth_left, depth_right)
        },
    }
}