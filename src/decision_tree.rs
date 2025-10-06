use std::collections::HashMap;
use crate::types;

use types::{Datapoint, DecisionTree, AttrType,
DesiredClassType, AttrGet, DesiredClassGet,
DatapointSplitterFn, DatapointSplitter, AttrDict };

pub static N_WAY_SPLIT : usize = 2;
pub static MIN_LEAF_SIZE : usize = 1;

pub static mut N_CALLS : usize = 0;
pub static mut N_LEAFS : usize = 0;
pub static mut N_INPURE_LEAFS : usize = 0;

pub fn get_binary_splitter(attribute : AttrGet, pivot : AttrType) -> DatapointSplitter {
    let splitter_fn : DatapointSplitterFn = |datapoint, piv, attr : AttrGet| {
        if attr(datapoint) <= piv {0} else {1}
    };
    return (pivot, attribute, splitter_fn);
}

pub fn gini_index(desired_classes_set : &Vec<DesiredClassType>) -> f64 {

    let mut desired_class_elements : HashMap<DesiredClassType, i32> = HashMap::new();
    for desired_class_element in desired_classes_set {
        let entry = desired_class_elements.entry(*desired_class_element).or_insert(0);
        *entry += 1;
    }

    let set_len = desired_classes_set.len() as f64;
    let mut sum = 1.0;

    for (_, quantity) in desired_class_elements {
        let prob = quantity as f64 / set_len;
        sum -= prob * prob;
    }

    return sum;
}

pub fn get_inpurity_reduction(
    set : &Vec<Datapoint>, attribute : AttrGet, candidate : AttrType, desired_class : DesiredClassGet) -> f64 {

    let (_, _, splitter_fn) = get_binary_splitter(attribute, candidate);

    let mut child_sets : Vec<Vec<DesiredClassType>> = vec![];
    for _ in 0..N_WAY_SPLIT {child_sets.push(vec![]);}

    for datapoint in set {
        let way = splitter_fn(datapoint, candidate, attribute);
        let desired_class_datapoint = desired_class(datapoint);

        child_sets[way].push(desired_class_datapoint);
    }

    let mut ds = 0.0;//gini_index(&set_);
    let set_len = set.len() as f64;

    for way in 0..N_WAY_SPLIT {
        let ratio : f64 = child_sets[way].len() as f64 / set_len as f64;
        ds -= ratio * gini_index(&child_sets[way]);
    }

    return ds;
}

// First Middle Last (FML)
pub fn get_i32_candidates_FML(set : &Vec<Datapoint>, attribute : AttrGet) -> Vec<AttrType> {

    if set.is_empty() {
        panic!("Empty set when looking for candidates!");
    }

    let a = 0;
    let c = set.len() - 1;
    let b = (a + c)/2;

    let candidates = vec![attribute(&set[a]), attribute(&set[b]), attribute(&set[c])];

    return candidates;
}

// Minimum Mean Maximum (MMM)
pub fn get_i32_candidates_MMM(set : &Vec<Datapoint>, attribute : AttrGet) -> Vec<AttrType> {

    if set.is_empty() {
        panic!("Empty set when looking for candidates!");
    }

    let mut min = attribute(&set[0]);
    let mut max = attribute(&set[0]);

    for datapoint in set {
        let value = attribute(datapoint);
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }

    let a = (min + max)/4;
    let mean = (min + max)/2;
    let b = (3 * (min + max))/4;

    let candidates = vec![a, mean, b];

    return candidates;
}

pub fn get_best_attribute_splitter(
    set : &Vec<Datapoint>, attribute : AttrGet, desired_class : DesiredClassGet) -> (f64, DatapointSplitter) {
    
    let candidates = get_i32_candidates_FML(set, attribute);

    if candidates.is_empty() {
        panic!("No suitable candidate found!");
    }

    let mut best_candidate = candidates[0];
    let mut best_inpurity_reduction = 1.0;

    for candidate in candidates {
        let inpurity_reduction = get_inpurity_reduction(set, attribute, candidate, desired_class);
        if inpurity_reduction >= best_inpurity_reduction || best_inpurity_reduction == 1.0 {
            best_inpurity_reduction = inpurity_reduction;
            best_candidate = candidate;
        }
    }

    return (best_inpurity_reduction, get_binary_splitter(attribute, best_candidate));
}

pub fn get_best_splitter(
    set : &Vec<Datapoint>, attributes : &AttrDict, desired_class : DesiredClassGet) -> DatapointSplitter {
    
    let mut best_inpurity_reduction = 1.0;
    let mut best_splitter : DatapointSplitter = types::_get_stub_splitter();

    for attribute in attributes.values() {

        let (inpurity_reduction, splitter) = get_best_attribute_splitter(set, *attribute, desired_class);

        if inpurity_reduction >= best_inpurity_reduction || best_inpurity_reduction == 1.0 {
            best_inpurity_reduction = inpurity_reduction;
            best_splitter = splitter;
        }
    }
    
    return best_splitter;
}

pub fn split_by_attribute(tree : &mut DecisionTree, splitter : DatapointSplitter) {

    let (pivot, attribute, splitter_fn) = splitter;
    tree.splitter = splitter;

    let mut children : Vec<DecisionTree> = vec![];
    for _ in 0..N_WAY_SPLIT {
        children.push(types::new_empty_tree());
    }

    let set_len = tree.set.len();
    for _ in 0..set_len {
        let datapoint = tree.set.remove(0);
        let way = splitter_fn(&datapoint, pivot, attribute);
        children[way].set.push(datapoint);
    }

    let mut n_non_empty_children = 0;
    let mut non_empty_child_index = 0;
    for i in 0..N_WAY_SPLIT {
        if children[i].set.len() > 0 {
            non_empty_child_index = i;
            n_non_empty_children += 1;
        }
    }

    if n_non_empty_children == 1 {
        tree.set = children.remove(non_empty_child_index).set;
        unsafe {N_INPURE_LEAFS += 1;}
    }
    else {
        tree.children = children;
    }
}

pub fn split(tree : &mut DecisionTree, attributes : &AttrDict, desired_class : DesiredClassGet) {

    unsafe {N_CALLS += 1;}

    if tree.set.len() <= MIN_LEAF_SIZE {
        unsafe {N_LEAFS += 1;}
        return;
    }

    let splitter = get_best_splitter(&tree.set, attributes, desired_class);
    split_by_attribute(tree, splitter);
    for mut child in &mut tree.children {
        split(&mut child, attributes, desired_class);
    }
}