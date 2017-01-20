extern crate petgraph;

use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use petgraph::algo::{toposort, is_cyclic_directed, connected_components};
use petgraph::dot::{Dot, Config};
use petgraph::visit::{Bfs, Walker};

use std::collections::{HashMap, HashSet};


fn main() {
    println!("Hello, graph!\n");

    let mut deps = Graph::<&str, &str>::new();
    let a = deps.add_node("A");
    let b = deps.add_node("B");
    let c = deps.add_node("C");
    let d = deps.add_node("D");
    let e = deps.add_node("E");
    let f = deps.add_node("F");
    let g = deps.add_node("G");
    let h = deps.add_node("H");

    deps.extend_with_edges(&[(a, c), (b, c), (c, f), (c, e), (d, e), (e, f), (g, h)]);

    println!("Input:");
    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));

    println!("Is cyclic: {}\n", is_cyclic_directed(&deps));

    let mut node_map: HashMap<usize, usize> = HashMap::new(); // Node -> Component #

    let undirected = deps.clone().into_edge_type::<Undirected>();
    println!("Connected components: {}", connected_components(&deps));
    let cc = petgraph::algo::kosaraju_scc(&undirected);
    for i in 0..cc.len() {
        print!("Component {}: ", i);
        let v = &cc[i];
        for n in v {
            print!("{} ", n.index());
            node_map.insert(n.index(), i);
        }
        println!("");
    }
    println!("");

    println!("BFS on element 0:");
    let bfs0 = Bfs::new(&deps, a);
    for k in bfs0.clone().iter(&deps) {
        print!("{:?} ", k.index());
    }
    println!("\n");

    println!("BFS on element 1:");
    let bfs1 = Bfs::new(&deps, b);
    for k in bfs1.clone().iter(&deps) {
        print!("{:?} ", k.index());
    }
    println!("\n");

    let t = match toposort(&deps, None) {
        Ok(ref v) => {
            println!("Topo sort:");
            for n in v {
                print!("{} ", n.index());
            }
            println!("");
            v.clone()
        }
        Err(c) => {
            panic!("Error: cycle found at {:?}", c.node_id());
        }
    };

    println!("\nBuilding node, component -> topo maps:");
    let mut comp_map: HashMap<usize, usize> = HashMap::new(); // Component # -> Topo sort start
    let mut topo_map: HashMap<usize, usize> = HashMap::new(); // Node -> Topo index

    let mut matched: i32 = -1;
    for i in 0..t.len() {
        println!("Node {} -> Topo index {}", t[i].index(), i);
        topo_map.insert(t[i].index(), i);

        let comp_num = *node_map.get(&t[i].index()).unwrap();
        if comp_num as i32 != matched {
            println!("Component {} -> Topo index {}", comp_num, i);
            comp_map.insert(comp_num as usize, i);
            matched = comp_num as i32;
        }
    }

    println!("\nFinding the ordering for node 1:");
    let start: usize = 1;
    let mut curr: usize = *topo_map.get(&start).unwrap();

    let mut bfs_set: HashSet<NodeIndex> = bfs1.iter(&deps).collect();

    while !bfs_set.is_empty() {
        if bfs_set.contains(&t[curr]) {
            print!("{} ", t[curr].index());
            bfs_set.remove(&t[curr]);
        }
        curr = curr + 1;
    }
    println!("");
}