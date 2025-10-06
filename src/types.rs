use std::collections::HashMap;

pub type AttrType = i32; // FIX: I'm assuming all attribute types are the same, this isn't ideal
pub type DesiredClassType = (i32, i32);
pub type AttrGet = fn(&Datapoint) -> AttrType;
pub type DesiredClassGet = fn(&Datapoint) -> DesiredClassType;
pub type DatapointSplitterFn = fn(&Datapoint, AttrType, AttrGet) -> usize;
pub type DatapointSplitter = (AttrType, AttrGet, DatapointSplitterFn);
pub type AttrDict = HashMap<i32, AttrGet>;

pub struct Datapoint {
    pub red : i32,
    pub green : i32,
    pub blue : i32,
    pub x : i32,
    pub y : i32,
}

pub struct DecisionTree {
    pub set : Vec<Datapoint>,
    pub children : Vec<DecisionTree>,
    pub splitter : DatapointSplitter
}

pub fn rgb_datapoint(red : i32, green : i32, blue : i32, x : i32, y : i32) -> Datapoint {
    Datapoint{red : red, green : green, blue : blue, x : x, y : y}
}

pub fn new_empty_tree() -> DecisionTree {
    DecisionTree{set : vec![], children : vec![], splitter : _get_stub_splitter()}
}

pub fn _get_stub_splitter() -> DatapointSplitter {
    (0, |_|{0}, |_, _, _| {0})
}

pub fn print_tree(tree : &DecisionTree, tab : i32) {

    let mut tab_str = String::new();
    for _ in 0..tab {
        tab_str.push_str("\t");
    }

    print!("{tab_str}Node:");
    for datapoint in &tree.set {
        print!(" {:?} {:?}", datapoint.x, datapoint.y);
    }
    let ex_datapoint = Datapoint{red : 0, green : 1, blue : 2, x : -1, y : -1};
    print!("| x <= {} taking into account channel{} ", tree.splitter.0, tree.splitter.1(&ex_datapoint));

    println!("");
    for child in &tree.children {
        println!("{tab_str}My Children: ");
        print_tree(child, tab + 1);
    }
}

pub fn print_tree_stats(tree : &DecisionTree, depth : i32) -> i32 {

    let mut max_depth = depth;

    for child in &tree.children {
        max_depth = std::cmp::max(print_tree_stats(child, depth + 1), max_depth);
    }

    return max_depth;
}