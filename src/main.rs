use std::collections::HashMap;

mod types;
mod image_reader;
mod decision_tree;

use types::{DecisionTree, DesiredClassGet, AttrDict};


fn main() {

    let mut attributes : AttrDict = HashMap::new();
    attributes.insert(0, |datapoint|{datapoint.x});
    attributes.insert(1, |datapoint|{datapoint.y});
    
    let desired_class : DesiredClassGet = |datapoint|{(
        datapoint.red, datapoint.green, datapoint.blue)};

    let dataset = image_reader::image_to_pixels("images/small4.png");

    let mut tree : DecisionTree = DecisionTree{
        set : dataset,
        children : vec![],
        splitter : types::_get_stub_splitter()
    };

    decision_tree::split(&mut tree, &attributes, desired_class);

    println!("CALLS: {}", unsafe {decision_tree::N_CALLS});
    println!("LEAFS: {}", unsafe {decision_tree::N_LEAFS + decision_tree::N_INPURE_LEAFS});
    println!("INPURE_LEAFS: {}", unsafe {decision_tree::N_INPURE_LEAFS});
    println!("depth: {}", types::print_tree_stats(&tree, 0));
    //print_tree(&tree, 0);
}