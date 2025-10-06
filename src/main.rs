use std::collections::HashMap;
use std::env;

mod types;
mod image_reader;
mod decision_tree;

use types::{DecisionTree, DesiredClassGet, AttrDict};

fn print_stats(tree : &DecisionTree, data_bytes : usize) {

    let bytes_per_node : i32 = 1;
    let bytes_per_leaf : i32 = 3;
    let original_info = 1 + ((0.0/3.0) * (data_bytes as f64)) as i32;

    let n_leafs : i32 = unsafe{decision_tree::N_LEAFS + decision_tree::N_INPURE_LEAFS} as i32;
    let n_nodes : i32 = unsafe{decision_tree::N_CALLS as i32 - n_leafs as i32};

    let theoretical_byte_usage = 
        bytes_per_node * n_nodes + bytes_per_leaf * n_leafs + original_info;

    println!("BYTES: {} / {} = {}", theoretical_byte_usage, data_bytes,
     theoretical_byte_usage as f64/data_bytes as f64);
    
    println!("CALLS (NODES): {}", unsafe {decision_tree::N_CALLS});
    println!("LEAFS: {}", unsafe {decision_tree::N_LEAFS + decision_tree::N_INPURE_LEAFS});
    println!("INPURE_LEAFS: {}", unsafe {decision_tree::N_INPURE_LEAFS});
    println!("depth: {}", types::print_tree_stats(&tree, 0));

    //print_tree(&tree, 0);
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Image path not supplied!")
    }

    let mut attributes : AttrDict = HashMap::new();
    attributes.insert(0, |datapoint|{datapoint.x});
    attributes.insert(1, |datapoint|{datapoint.y});
    
    let desired_class : DesiredClassGet = |datapoint|{(datapoint.red, datapoint.green, datapoint.blue)};

    let dataset = image_reader::image_to_pixels(args[1].as_str());
    let data_bytes = dataset.len() * 3;

    let mut tree : DecisionTree = DecisionTree{
        set : dataset,
        children : vec![],
        splitter : types::_get_stub_splitter()
    };

    decision_tree::split(&mut tree, &attributes, desired_class);

    print_stats(&tree, data_bytes);
}