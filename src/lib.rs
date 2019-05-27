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
pub fn draw_something(canvas_id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
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

    // draw_tree(context, 4);


    let rendering_tree = to_rendering_tree(&TournamentTree::Node {
        left: Box::new(TournamentTree::Node {
            left: Box::new(TournamentTree::Empty),
            right: Box::new(TournamentTree::Empty),            
        }),
        right: Box::new(TournamentTree::Node {
            left: Box::new(TournamentTree::Node {
                left: Box::new(TournamentTree::Empty),
                right: Box::new(TournamentTree::Empty),            
            }),
            right: Box::new(TournamentTree::Empty),            
        }),
    }, 0, 500, 0, 50);
    render_tree(&rendering_tree, &context);

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

pub fn tree_boundary_height(base_height: u32, tree_depth: u32) -> u32 {
    2u32.pow(tree_depth - 1) * base_height
}

pub enum RenderingTree {
    Empty,
    Node { left: Box<RenderingTree>, right: Box<RenderingTree>, top: u32, bottom: u32, edge: u32, gap: u32 },
}

pub fn to_rendering_tree(root: &TournamentTree, top: u32, bottom: u32, edge: u32, gap: u32) -> RenderingTree {
    match root {
        TournamentTree::Empty => RenderingTree::Empty,
        TournamentTree::Node { left, right } => {
            RenderingTree::Node {
                left: Box::new(to_rendering_tree(left, top, (top + bottom) / 2, edge + gap, gap)),
                right: Box::new(to_rendering_tree(right, (top + bottom) / 2, bottom, edge + gap, gap)),
                top: top,
                bottom: bottom,
                edge: edge,
                gap: gap,
            }
        }
    }
}

pub fn render_tree(root: &RenderingTree, context: &web_sys::CanvasRenderingContext2d) {

    match root {
        RenderingTree::Empty => {},
        RenderingTree::Node { left, right, top, bottom, edge, gap } => {
            context.stroke_rect(*edge as f64, *top as f64, *gap as f64, (bottom - top) as f64);
            render_tree(left, context);
            render_tree(right, context);
        }
    }

}