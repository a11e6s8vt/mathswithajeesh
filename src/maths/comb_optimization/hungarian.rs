extern crate petgraph;

use crate::graph_theory::bipartite::BipartiteGraph;
use petgraph::data::DataMap;
use petgraph::graph::IndexType;
use petgraph::graph::*;
use petgraph::visit::{Dfs, GraphRef, IntoNeighbors, IntoNodeIdentifiers, NodeCount, NodeIndexable, VisitMap, Visitable};
use petgraph::*;
use rocket::{
        self,
        serde::{json::Json, Deserialize, Serialize},
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;

use rocket::response::stream::Event;
use serde_json::{Map, Value};

pub fn hungarian_maximum_matching<G: DataMap, N, VM>(g: G, mut node_index_weight_map: HashMap<usize, String>, matched_edges: HashSet<(N, N)>, g_vertex_u_v_sets: &BipartiteGraph<N>) -> Option<(Vec<N>, HashSet<(N, N)>)>
where
        G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N> + IntoNodeIdentifiers<NodeId = N> + NodeCount + NodeIndexable + Serialize + std::fmt::Debug,
        N: Default + IndexType + Copy + Clone + PartialEq + Eq + std::hash::Hash + Serialize + std::fmt::Debug,
        VM: VisitMap<N>,
{
        let nodes_u: HashSet<N> = g_vertex_u_v_sets.nodes_u.iter().copied().collect();
        // println!("nodes_u = {:?}", nodes_u);
        let nodes_v: HashSet<N> = g_vertex_u_v_sets.nodes_v.iter().copied().collect();

        let mut hungarian_states = Map::new();

        // Thm: Given a bipartite graph G = (S, T, E), the Hungarian algorithm returns a
        // matching M and a cover K with |M| = |K| in time O(n^3)
        // Set min cover size to zero to start with
        // let mut min_cover_size = 0;

        // // set maximum matching size to the size of the given matching
        // let mut max_matching_size = m.len();

        // Function return min_cover and max_matching
        let mut min_cover: Vec<N> = Vec::new();
        let mut max_matching: HashSet<(N, N)> = HashSet::new();
        let mut covered_nodes: HashSet<N> = HashSet::new();

        for edge in &matched_edges {
                // println!("m edge = {:?}", edge);
                max_matching.insert(*edge);
        }

        // println!("covered_nodes = {:?}", covered_nodes);

        let mut red = g.visit_map();
        let mut blue = g.visit_map();
        let mut scanned = g.visit_map();
        let mut exposed_nodes_u = g.visit_map();
        let mut exposed_nodes_v = g.visit_map();

        /*
         * Below block is to describe the solution step
         */
        let mut start_state = Map::new();
        start_state.insert(
                "input_graph".to_string(),
                Value::String(serde_json::to_string(&g).unwrap()),
        );
        let matching_str = max_matching.iter().cloned().collect::<Vec<(N, N)>>();
        start_state.insert(
                "matching".to_string(),
                Value::String(serde_json::to_string(&matching_str).unwrap()),
        );
        start_state.insert("blue".to_string(), Value::Array(vec![]));
        start_state.insert("red".to_string(), Value::Array(vec![]));
        start_state.insert("scanned".to_string(), Value::Array(vec![]));
        let description = "Algorithm starts with the given bipartite input_graph `g` and a matching `m`. All the vertices are uncolored \
        and unscanned (blue, red and scanned are empty). Go to Step `Exposed Vertex`.";
        start_state.insert(
                "description".to_string(),
                Value::String(description.to_string()),
        );
        hungarian_states.insert("start".to_string(), Value::Object(start_state));
        /*
         * Above block is to describe the solution step
         */

        // println!("hungarian_states = {:?}", hungarian_states);

        'hungarian: loop {
                for edge in max_matching.iter() {
                        // println!("m edge = {:?}", edge);
                        let node1 = edge.0;
                        let node2 = edge.1;
                        covered_nodes.insert(node1);
                        covered_nodes.insert(node2);
                }

                for node in &nodes_u {
                        if !covered_nodes.contains(&node) {
                                exposed_nodes_u.visit(*node);
                        }
                }

                for node in &nodes_v {
                        if !covered_nodes.contains(&node) {
                                exposed_nodes_v.visit(*node);
                        }
                }

                let mut node_ids = g.node_identifiers().peekable();
                // println!("graph = {:?}", &g);

                // let mut exposed_vertex_state_counter = 1;

                'exposed_vertex: while let Some(node_id) = node_ids.next() {
                        // println!("node_id = {:?} {:?}", node_id, node_index_weight_map.get(&node_id.index()).unwrap());
                        let is_exposed_u = exposed_nodes_u.is_visited(&node_id);
                        let mut is_blue = blue.is_visited(&node_id);

                        // /*
                        // * Below block is to describe the solution step
                        // */
                        // let mut state_exposed_vertex = Map::new();

                        // state_exposed_vertex.insert(format!("Step {}", exposed_vertex_state_counter), Value::String(format!("Check if there is an uncolored exposed vertex in S")));
                        // exposed_vertex_state_counter += 1;
                        // /*
                        //  * Above block is to describe the solution step
                        //  */
                        match (is_exposed_u, is_blue) {
                                (true, false) => {
                                        let node_root_weight = node_index_weight_map.get(&node_id.index()).unwrap();
                                        // /*
                                        // * Below block is to describe the solution step
                                        // */
                                        // state_exposed_vertex.insert(format!("Step {}", exposed_vertex_state_counter), Value::String(format!("Node `{}` in S is uncolored and exposed. Color `{}` as blue",
                                        //         node_root_weight, node_root_weight)));
                                        // exposed_vertex_state_counter += 1;
                                        // /*
                                        //  * Above block is to describe the solution step
                                        //  */
                                        blue.visit(node_id);

                                        // /*
                                        // * Below block is to describe the solution step
                                        // */
                                        // state_exposed_vertex.insert(format!("Step {}", exposed_vertex_state_counter), Value::String(format!("Start a rooted tree X with root `{}` and go to step Tree Building",
                                        //         node_root_weight)));
                                        // exposed_vertex_state_counter += 1;
                                        // /*
                                        //  * Above block is to describe the solution step
                                        //  */
                                        let mut tree: Graph<String, String, Directed> = Graph::new();
                                        // let root = tree.add_node("s".to_string() + &node.index().to_string());
                                        // println!("node weight = {}", g.node_weight(node.index()));
                                        // let n_index: N = node.index();
                                        let mut tree_paths: Vec<Vec<String>> = Vec::new();
                                        let root = tree.add_node(node_root_weight.clone());
                                        let mut tree_last_added_node = ::std::collections::VecDeque::new();
                                        tree_last_added_node.push_back(root);
                                        let mut graph_last_visited_blue_node = ::std::collections::VecDeque::new();
                                        graph_last_visited_blue_node.push_back(node_id);

                                        /*
                                         * Below block is to describe the solution step
                                         */
                                        let mut tree_building_state_counter = 1;
                                        let mut state_tree_building = Map::new();

                                        state_tree_building.insert(
                                                format!("Step {}", tree_building_state_counter),
                                                Value::String(serde_json::to_string(&tree).unwrap()),
                                        );
                                        tree_building_state_counter += 1;
                                        /*
                                         * Above block is to describe the solution step
                                         */
                                        'tree_building: while let Some(s) = graph_last_visited_blue_node.pop_front() {
                                                // /*
                                                // * Below block is to describe the solution step
                                                // */
                                                // state_tree_building.insert(format!("Step {}", tree_building_state_counter), Value::String(serde_json::to_string(&tree).unwrap()));
                                                // tree_building_state_counter += 1;
                                                // /*
                                                // * Above block is to describe the solution step
                                                // */
                                                let node_s_weight = node_index_weight_map.get(&s.index()).unwrap();
                                                println!("s = {:?}", node_s_weight);
                                                is_blue = blue.is_visited(&s);
                                                let is_scanned = scanned.is_visited(&s);
                                                // /*
                                                // * Below block is to describe the solution step
                                                // */
                                                // state_tree_building.insert(format!("Step {}", tree_building_state_counter), Value::String("Check if there is an unscanned blue vertex in S.
                                                //         If there is no such vertex, go to Exposed Vertex step".to_string()));
                                                // tree_building_state_counter += 1;
                                                // /*
                                                // * Above block is to describe the solution step
                                                // */
                                                match (is_scanned, is_blue) {
                                                        (false, true) => {
                                                                // /*
                                                                // * Below block is to describe the solution step
                                                                // */
                                                                // state_tree_building.insert(format!("Step {}", tree_building_state_counter), Value::String(format!("Node '{}' is an unscanned blue vertex", node_s_weight)));
                                                                // tree_building_state_counter += 1;
                                                                // /*
                                                                // * Above block is to describe the solution step
                                                                // */
                                                                // Sort neighbours of  's' in ascending order
                                                                let mut neighbours_of_s: Vec<N> = g.neighbors(s).map(|neighbor| neighbor.clone()).collect();
                                                                neighbours_of_s.sort_by(|node_x, node_y| node_x.index().cmp(&node_y.index()));
                                                                // println!("neighbours_of_s => {:?}", neighbours_of_s);
                                                                // /*
                                                                // * Below block is to describe the solution step
                                                                // */
                                                                // state_tree_building.insert(format!("Step {}", tree_building_state_counter), Value::Array(format!("Neighbors of node '{}' are {}",
                                                                //         node_s_weight, serde_json::to_string(&neighbours_of_s).unwrap())));
                                                                // tree_building_state_counter += 1;
                                                                // /*
                                                                // * Above block is to describe the solution step
                                                                // */
                                                                for w in neighbours_of_s {
                                                                        let is_exposed_v = exposed_nodes_v.is_visited(&w);
                                                                        match is_exposed_v {
                                                                                true => {
                                                                                        // this should be storing all augmenting path options
                                                                                        let node_w_weight = node_index_weight_map.get(&w.index()).unwrap();
                                                                                        // println!("w = {:?}", node_w_weight);
                                                                                        // println!("w = {:?} is exposed", node_w_weight);
                                                                                        let t_node_w = tree.add_node(node_w_weight.clone());

                                                                                        // Adding the tree edge
                                                                                        let mut dfs = Dfs::new(&tree, root);
                                                                                        while let Some(nx) = dfs.next(&tree) {
                                                                                                if tree.node_weight(nx).unwrap() == node_s_weight {
                                                                                                        tree.add_edge(nx, t_node_w, "".to_string());
                                                                                                }
                                                                                        }

                                                                                        println!("tree = {:?}", tree);
                                                                                        // longest path algm
                                                                                        let mut path: Vec<String> = Vec::new();
                                                                                        let mut seen = tree.visit_map();
                                                                                        let mut processed_path = tree.visit_map();
                                                                                        seen.visit(root);
                                                                                        processed_path.visit(root);
                                                                                        let mut all_nodes_visited = false;
                                                                                        loop {
                                                                                                if all_nodes_visited == true {
                                                                                                        break;
                                                                                                }

                                                                                                let mut dfs = Dfs::new(&tree, root);
                                                                                                dfs.next(&tree);
                                                                                                let mut prev = root;
                                                                                                while let Some(nx) = dfs.next(&tree) {
                                                                                                        if !seen.is_visited(&nx) {
                                                                                                                match tree.contains_edge(prev, nx) {
                                                                                                                        true => {
                                                                                                                                seen.visit(nx);
                                                                                                                                prev = nx;
                                                                                                                        }
                                                                                                                        false => {
                                                                                                                                break;
                                                                                                                        }
                                                                                                                }
                                                                                                        }
                                                                                                }

                                                                                                let mut tree_nodes = tree.node_indices().peekable();
                                                                                                while let Some(t_node) = tree_nodes.next() {
                                                                                                        if seen.is_visited(&t_node) {
                                                                                                                if tree_nodes.peek().is_none() {
                                                                                                                        println!("last node = {:?}", t_node);
                                                                                                                        all_nodes_visited = true;
                                                                                                                }

                                                                                                                if !processed_path.is_visited(&t_node) || t_node == root {
                                                                                                                        if let Some(t_node_weight) = tree.node_weight(t_node) {
                                                                                                                                path.push(t_node_weight.clone());
                                                                                                                                processed_path.visit(t_node);
                                                                                                                        } else {
                                                                                                                                panic!("Node do not have weight!");
                                                                                                                        }
                                                                                                                }
                                                                                                        }
                                                                                                }
                                                                                                tree_paths.push(path.clone());
                                                                                                path.clear();
                                                                                        }

                                                                                        // aug_path_nodes contains node weight like s1, s2, t1, t2
                                                                                        let mut aug_path_nodes = vec!["".to_string(); 0];
                                                                                        println!("aug_path_nodes = {:?}", aug_path_nodes);
                                                                                        for item in &tree_paths {
                                                                                                if item.len() > aug_path_nodes.len() {
                                                                                                        aug_path_nodes = item.clone();
                                                                                                }
                                                                                        }
                                                                                        println!("aug_path_nodes = {:?}", aug_path_nodes);

                                                                                        let mut aug_path_edges = vec![("".to_string(), "".to_string()); 0];

                                                                                        for (i, id1) in aug_path_nodes.iter().enumerate() {
                                                                                                if let Some(id2) = aug_path_nodes.get(i + 1) {
                                                                                                        aug_path_edges.push((id1.clone(), id2.clone()));
                                                                                                }
                                                                                        }
                                                                                        println!("aug_path_edges = {:?}", aug_path_edges);
                                                                                        let mut n1_index = 0;
                                                                                        let mut n2_index = 0;
                                                                                        let mut aug_path: HashSet<(N, N)> = HashSet::new();
                                                                                        for edge in aug_path_edges {
                                                                                                println!("edge = {:?}", edge);
                                                                                                let n1_weight = edge.0;
                                                                                                let n2_weight = edge.1;

                                                                                                for (n_index, n_weight) in node_index_weight_map.iter_mut() {
                                                                                                        if n1_weight == *n_weight {
                                                                                                                n1_index = n_index.clone();
                                                                                                        } else if n2_weight == *n_weight {
                                                                                                                n2_index = n_index.clone();
                                                                                                        }
                                                                                                }

                                                                                                if g_vertex_u_v_sets.nodes_v.contains(&g.from_index(n1_index)) {
                                                                                                        aug_path.insert((g.from_index(n2_index), g.from_index(n1_index)));
                                                                                                } else {
                                                                                                        aug_path.insert((g.from_index(n1_index), g.from_index(n2_index)));
                                                                                                }
                                                                                        }

                                                                                        //let max_matching = max_matching.clone();
                                                                                        println!("aug_path = {:?}", aug_path);
                                                                                        // /*
                                                                                        // * Below block is to describe the solution step
                                                                                        // */
                                                                                        // state_tree_building.insert(format!("Step {}", tree_building_state_counter), Value::String(format!("Node '{}' of T is exposed. The new tree is, X = {}. We have an augmenting
                                                                                        //         path P = {}. Go to step Augment", node_w_weight, serde_json::to_string(&tree).unwrap(), serde_json::to_string(&aug_path).unwrap())));
                                                                                        // tree_building_state_counter += 1;
                                                                                        // /*
                                                                                        // * Above block is to describe the solution step
                                                                                        // */
                                                                                        println!("max_matching = {:?}", max_matching);
                                                                                        max_matching = max_matching.symmetric_difference(&aug_path).copied().map(|x| x).collect();
                                                                                        println!("new match = {:?}", &max_matching);

                                                                                        covered_nodes.clear();
                                                                                        // for edge in &max_matching {
                                                                                        //         println!("max edge = {:?}", edge);
                                                                                        // }
                                                                                        g.reset_map(&mut blue);
                                                                                        g.reset_map(&mut red);
                                                                                        g.reset_map(&mut scanned);
                                                                                        g.reset_map(&mut exposed_nodes_u);
                                                                                        g.reset_map(&mut exposed_nodes_v);

                                                                                        // /*
                                                                                        // * Below block is to describe the solution step
                                                                                        // */
                                                                                        // let mut augment_state_counter = 1;
                                                                                        // let mut state_augment = Map::new();

                                                                                        // state_augment.insert(format!("Step {}", augment_state_counter), Value::String(format!("The new matching is given by M = M Δ P = {} \n
                                                                                        //         Remove all colors and mark each vertex as unscanned and go to step Exposed Vertex.",
                                                                                        //         serde_json::to_string(&max_matching).unwrap())));
                                                                                        // augment_state_counter += 1;
                                                                                        // /*
                                                                                        // * Above block is to describe the solution step
                                                                                        // */
                                                                                        continue 'hungarian;
                                                                                }
                                                                                false => {
                                                                                        let is_red = red.is_visited(&w);
                                                                                        if !is_red {
                                                                                                red.visit(w);
                                                                                                if let Some(uw_edge) = max_matching.iter().find(|&x| x.1.index() == w.index()) {
                                                                                                        let u = uw_edge.0;
                                                                                                        blue.visit(u);
                                                                                                        let node_w_weight = node_index_weight_map.get(&w.index()).unwrap();
                                                                                                        // println!("w = {:?}", node_w_weight);
                                                                                                        let t_node_w = tree.add_node(node_w_weight.clone());
                                                                                                        let node_u_weight = node_index_weight_map.get(&u.index()).unwrap();
                                                                                                        // println!("u = {:?}", node_u_weight);
                                                                                                        let t_node_u = tree.add_node(node_u_weight.clone());

                                                                                                        // Adding the tree edge
                                                                                                        let mut dfs = Dfs::new(&tree, root);
                                                                                                        while let Some(nx) = dfs.next(&tree) {
                                                                                                                if tree.node_weight(nx).unwrap() == node_s_weight {
                                                                                                                        tree.add_edge(nx, t_node_w, "".to_string());
                                                                                                                }
                                                                                                        }
                                                                                                        tree.add_edge(t_node_w, t_node_u, "".to_string());

                                                                                                        scanned.visit(node_id);
                                                                                                        graph_last_visited_blue_node.push_back(u);
                                                                                                        // println!("graph_last_visited_blue_node = {:?}", &graph_last_visited_blue_node);
                                                                                                        // /*
                                                                                                        // * Below block is to describe the solution step
                                                                                                        // */
                                                                                                        // state_tree_building.insert(format!("Step {}", tree_building_state_counter), Value::String(format!("Node '{}' of T is not exposed and uncolored
                                                                                                        //         We color it red. Colored the node u = {} blue. Since the edge uw = '{}{}' ∈ M, we add vertices w = {} and u = {} to the tree and then we add
                                                                                                        //         the edges sw = {}{} and wu = {}{} to the tree. We mark s = {} as scanned and continue with tree building step.",
                                                                                                        //         node_w_weight, node_u_weight, node_u_weight, node_w_weight, node_w_weight, node_u_weight, node_s_weight, node_w_weight,
                                                                                                        //         node_w_weight, node_u_weight, node_s_weight)));
                                                                                                        // tree_building_state_counter += 1;
                                                                                                        // /*
                                                                                                        // * Above block is to describe the solution step
                                                                                                        // */
                                                                                                        continue 'tree_building;
                                                                                                }
                                                                                        }
                                                                                }
                                                                        }
                                                                }
                                                        }
                                                        (_, _) => {
                                                                if graph_last_visited_blue_node.is_empty() {
                                                                        continue 'exposed_vertex;
                                                                }
                                                        }
                                                }
                                        }
                                }
                                (_, _) => {
                                        if node_ids.peek().is_none() {
                                                println!("No uncolored exposed vertex in S");
                                                break 'hungarian;
                                        } else {
                                                continue 'exposed_vertex;
                                        }
                                }
                        }
                }
        }

        // min_cover contains uncolored vertices from S and colored vertices from T in G(S, T)
        for node in &g_vertex_u_v_sets.nodes_u {
                if !blue.is_visited(&node) {
                        min_cover.push(*node);
                }
        }

        for node in &g_vertex_u_v_sets.nodes_v {
                if red.is_visited(&node) {
                        min_cover.push(*node);
                }
        }

        Some((min_cover, max_matching))
}
