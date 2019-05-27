//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate hinoki_wasm;
use hinoki_wasm::TournamentTree;
use hinoki_wasm::depth_of_tree;
use hinoki_wasm::tree_boundary_height;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn depth_of_empty_tree() {
    let root = TournamentTree::Empty;
    let depth = depth_of_tree(&root);
    assert_eq!(depth, 0);
}

#[wasm_bindgen_test]
fn depth_of_single_node_tree() {
    let root = TournamentTree::Node { 
        left: Box::new(TournamentTree::Empty), 
        right: Box::new(TournamentTree::Empty),
        tournament_match: Default::default(),
    };
    let depth = depth_of_tree(&root);
    assert_eq!(depth, 1);
}

#[wasm_bindgen_test]
fn depth_of_bigger_tree() {
    let root = TournamentTree::Node { 
        left: Box::new(TournamentTree::Node {
            left: Box::new(TournamentTree::Empty),
            right: Box::new(TournamentTree::Empty),
            tournament_match: Default::default(),
        }), 
        right: Box::new(TournamentTree::Empty),
        tournament_match: Default::default(),
    };
    let depth = depth_of_tree(&root);
    assert_eq!(depth, 2);
}

#[wasm_bindgen_test]
fn tree_boundary_height_single() {
    let height = tree_boundary_height(10, 1);
    assert_eq!(height, 10) 
}

#[wasm_bindgen_test]
fn tree_boundary_height_double() {
    let height = tree_boundary_height(10, 2);
    assert_eq!(height, 20) 
}