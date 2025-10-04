use std::collections::HashMap;
use image::GenericImageView;

type AttrType = i32; // FIX: I'm assuming all attribute types are the same, this isn't ideal
type AttrGet = fn(&Datapoint) -> AttrType;
type DatapointSplitterFn = fn(&Datapoint, AttrType, AttrGet) -> usize;
type DatapointSplitter = (AttrType, AttrGet, DatapointSplitterFn);
type AttrDict = HashMap<i32, AttrGet>;

static N_WAY_SPLIT : usize = 2;
static MIN_LEAF_SIZE : usize = 1;

static mut N_CALLS : usize = 0;
static mut N_LEAFS : usize = 0;

struct Datapoint {
    red : i32,
    green : i32,
    blue : i32,
    index : i32,
}

fn take_set_from_child(child : DecisionTree) -> Vec<Datapoint> {
    child.set
}

fn rgb_datapoint(red : i32, green : i32, blue : i32, index : i32) -> Datapoint {
    Datapoint{red : red, green : green, blue : blue, index : index}
}

struct DecisionTree {
    set : Vec<Datapoint>,
    children : Vec<DecisionTree>,
    splitter : DatapointSplitter
}

fn new_empty_tree() -> DecisionTree {
    DecisionTree{set : vec![], children : vec![], splitter : _get_stub_splitter()}
}

fn print_tree(tree : &DecisionTree, tab : i32) {

    let mut tab_str = String::new();
    for _ in 0..tab {
        tab_str.push_str("\t");
    }

    print!("{tab_str}Node:");
    for datapoint in &tree.set {
        print!(" {:?}", datapoint.index);
    }
    let ex_datapoint = Datapoint{red : 0, green : 1, blue : 2, index : -1};
    print!("| x <= {} taking into account channel{} ", tree.splitter.0, tree.splitter.1(&ex_datapoint));

    println!("");
    for child in &tree.children {
        println!("{tab_str}My Children: ");
        print_tree(child, tab + 1);
    }
}

fn print_tree_stats(tree : &DecisionTree, depth : i32) -> i32 {

    let mut max_depth = depth;

    for child in &tree.children {
        max_depth = std::cmp::max(print_tree_stats(child, depth + 1), max_depth);
    }

    return max_depth;
}

fn _get_stub_splitter() -> DatapointSplitter {
    (0, |_|{0}, |_, _, _| {0})
}

fn get_binary_splitter(attribute : AttrGet, pivot : AttrType) -> DatapointSplitter {
    let splitter_fn : DatapointSplitterFn = |datapoint, piv, attr : AttrGet| {
        if attr(datapoint) <= piv {0} else {1}
    };
    return (pivot, attribute, splitter_fn);
}

fn gini_index(desired_classes_set : &Vec<AttrType>) -> f64 {

    let mut desired_class_elements : HashMap<AttrType, i32> = HashMap::new();
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

fn get_inpurity_reduction(
    set : &Vec<Datapoint>, attribute : AttrGet, candidate : AttrType, desired_class : AttrGet) -> f64 {

    let (_, _, splitter_fn) = get_binary_splitter(attribute, candidate);

    let mut child_sets : Vec<Vec<AttrType>> = vec![];
    for _ in 0..N_WAY_SPLIT {child_sets.push(vec![]);}
    let mut set_ : Vec<AttrType> = vec![];

    for datapoint in set {
        let way = splitter_fn(datapoint, candidate, attribute);
        let desired_class_datapoint = desired_class(datapoint);

        child_sets[way].push(desired_class_datapoint);
        set_.push(desired_class_datapoint);
    }

    let mut ds = 0.0;//gini_index(&set_);
    let set_len = set_.len() as f64;

    for way in 0..N_WAY_SPLIT {
        let ratio : f64 = child_sets[way].len() as f64 / set_len as f64;
        ds -= ratio * gini_index(&child_sets[way]);
    }

    return ds;
}

fn get_i32_candidates(set : &Vec<Datapoint>, attribute : AttrGet) -> Vec<AttrType> {

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

fn get_best_attribute_splitter(
    set : &Vec<Datapoint>, attribute : AttrGet, desired_class : AttrGet) -> (f64, DatapointSplitter) {
    
    let candidates = get_i32_candidates(set, attribute);

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

fn get_best_splitter(
    set : &Vec<Datapoint>, attributes : &AttrDict, desired_class : AttrGet) -> DatapointSplitter {
    
    let mut best_inpurity_reduction = 1.0;
    let mut best_splitter : DatapointSplitter = _get_stub_splitter();

    for attribute in attributes.values() {

        let (inpurity_reduction, splitter) = get_best_attribute_splitter(set, *attribute, desired_class);

        if inpurity_reduction >= best_inpurity_reduction || best_inpurity_reduction == 1.0 {
            best_inpurity_reduction = inpurity_reduction;
            best_splitter = splitter;
        }
    }
    
    return best_splitter;
}

fn split_by_attribute(tree : &mut DecisionTree, splitter : DatapointSplitter) {

    let (pivot, attribute, splitter_fn) = splitter;
    tree.splitter = splitter;

    let mut children : Vec<DecisionTree> = vec![];
    for _ in 0..N_WAY_SPLIT {
        children.push(new_empty_tree());
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
        tree.set = take_set_from_child(children.remove(non_empty_child_index));
    }
    else {
        tree.children = children;
    }
}

fn split(tree : &mut DecisionTree, attributes : &AttrDict, desired_class : AttrGet) {

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

fn image_to_pixels(filepath : &str) -> Vec<Datapoint> {
    let img = image::open(filepath).unwrap();
    let pixels = img.pixels();
    let (w, _) = img.dimensions();

    let mut vec : Vec<Datapoint> = vec![];
    
    for pixel in pixels {
        let color = pixel.2.0;
        let pos = (pixel.0, pixel.1);
        vec.push(rgb_datapoint(
            color[0] as i32, color[1] as i32, color[2] as i32,
            (pos.0 + w * pos.1) as i32));
    }

    println!("collected all pixels");

    return vec;
}

fn main() {

    let mut attributes : AttrDict = HashMap::new();
    attributes.insert(0, |datapoint|{datapoint.red});
    attributes.insert(1, |datapoint|{datapoint.green});
    attributes.insert(2, |datapoint|{datapoint.blue});
    
    let desired_class : AttrGet = |datapoint|{datapoint.index};

    let dataset = image_to_pixels("small6.png");

    let mut tree : DecisionTree = DecisionTree{
        set : dataset,
        children : vec![],
        splitter : _get_stub_splitter()
    };

    split(&mut tree, &attributes, desired_class);

    println!("CALLS: {}", unsafe {N_CALLS});
    println!("LEAFS: {}", unsafe {N_LEAFS});
    println!("depth: {}", print_tree_stats(&tree, 0));
}