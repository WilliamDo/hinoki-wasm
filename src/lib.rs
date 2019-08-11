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

    let base_width = 200;
    let base_height = 120;

    let tournament_tree = &TournamentTree::Node {
        left: Box::new(TournamentTree::Node {
            left: Box::new(TournamentTree::Node {
                left: Box::new(TournamentTree::Empty),
                right: Box::new(TournamentTree::Empty),
                tournament_match: TournamentMatch {
                    player_1: "Timo Boll".to_string(),
                    player_2: "Jun Mizutani".to_string(),
                },         
            }),
            right: Box::new(TournamentTree::Node {
                left: Box::new(TournamentTree::Empty),
                right: Box::new(TournamentTree::Empty),            
                tournament_match: TournamentMatch {
                    player_1: "Zhang Jike".to_string(),
                    player_2: "Ma Lin".to_string(),
                },         
            }),
            tournament_match: TournamentMatch {
                player_1: "Timo Boll".to_string(),
                player_2: "Zhang Jike".to_string(),
            },                   
        }),
        right: Box::new(TournamentTree::Node {
            left: Box::new(TournamentTree::Node {
                left: Box::new(TournamentTree::Empty),
                right: Box::new(TournamentTree::Empty),
                tournament_match: TournamentMatch {
                    player_1: "Koki Niwa".to_string(),
                    player_2: "Xu Xin".to_string(),
                },         
            }),
            right: Box::new(TournamentTree::Node {
                left: Box::new(TournamentTree::Empty),
                right: Box::new(TournamentTree::Empty),
                tournament_match: TournamentMatch {
                    player_1: "Simon Gauzy".to_string(),
                    player_2: "Ma Long".to_string(),
                },                     
            }),
            tournament_match: TournamentMatch {
                player_1: "Koki Niwa".to_string(),
                player_2: "Ma Long".to_string(),
            },                     
        }),
        tournament_match: TournamentMatch {
            player_1: "Timo Boll".to_string(),
            player_2: "Ma Long".to_string(),
        },         
    };

    let tree_depth = depth_of_tree(&tournament_tree);
    let boundary_height = tree_boundary_height(base_height, tree_depth);
    let boundary_width = tree_boundary_width(base_width, tree_depth);
    let rendering_tree = to_rendering_tree(&tournament_tree, 0, boundary_height, boundary_width - base_width, base_width);
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

#[derive(Clone, Default)]
pub struct TournamentMatch {
    player_1: String,
    player_2: String,
}

pub enum TournamentTree {
    Empty,
    Node { 
        left: Box<TournamentTree>, 
        right: Box<TournamentTree>,
        tournament_match: TournamentMatch,
    },
}

pub fn depth_of_tree(root: &TournamentTree) -> u32 {
    match root {
        TournamentTree::Empty => 0,
        TournamentTree::Node { left, right, tournament_match: _ } => {
            let depth_left = depth_of_tree(&*left);
            let depth_right = depth_of_tree(&*right);
            1 + cmp::max(depth_left, depth_right)
        },
    }
}

pub fn tree_boundary_height(base_height: u32, tree_depth: u32) -> u32 {
    2u32.pow(tree_depth - 1) * base_height
}

pub fn tree_boundary_width(base_width: u32, tree_depth: u32) -> u32 {
    base_width * tree_depth
}

pub enum RenderingTree {
    Empty,
    Node { 
        left: Box<RenderingTree>, 
        right: Box<RenderingTree>, 
        y_top: u32, 
        y_bottom: u32, 
        x_left: u32, 
        width: u32, 
        tournament_match: TournamentMatch,
    },
}

fn to_rendering_tree(root: &TournamentTree, y_top: u32, y_bottom: u32, x_left: u32, width: u32) -> RenderingTree {
    match root {
        TournamentTree::Empty => RenderingTree::Empty,
        TournamentTree::Node { left, right, tournament_match } => {
            RenderingTree::Node {
                left: Box::new(to_rendering_tree(left, y_top, (y_top + y_bottom) / 2, x_left - width, width)),
                right: Box::new(to_rendering_tree(right, (y_top + y_bottom) / 2, y_bottom, x_left - width, width)),
                y_top: y_top,
                y_bottom: y_bottom,
                x_left: x_left,
                width: width,
                tournament_match: tournament_match.clone(),
            }
        }
    }
}

fn render_tree(root: &RenderingTree, context: &web_sys::CanvasRenderingContext2d) {

    match root {
        RenderingTree::Empty => {},
        RenderingTree::Node { left, right, y_top, y_bottom, x_left, width, tournament_match } => {
            // context.stroke_rect(*x_left as f64, *y_top as f64, *width as f64, (y_bottom - y_top) as f64);

            // todo pass in the height and width from elsewhere
            let container_height = 60;
            let container_width = 180;

            let y_center = (y_top + y_bottom) / 2;
            let x_center = x_left + width / 2;

            context.stroke_rect((x_center - container_width / 2).into(), (y_center - container_height / 2).into(), container_width.into(), container_height.into());

            context.set_text_align("center");

            context.fill_text(&tournament_match.player_1, x_center.into(), y_center.into());
            context.fill_text(&tournament_match.player_2, x_center.into(), (y_center + 10).into());

            render_tree(left, context);
            render_tree(right, context);
        }
    }

}