//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate hinoki_wasm;
use hinoki_wasm::TournamentTree;
use hinoki_wasm::depth_of_tree;

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
        right: Box::new(TournamentTree::Empty) 
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
        }), 
        right: Box::new(TournamentTree::Empty) 
    };
    let depth = depth_of_tree(&root);
    assert_eq!(depth, 2);
}
