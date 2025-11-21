#![allow(dead_code)]

extern crate nalgebra;
extern crate petgraph;

use crate::comb_optimization::hungarian::hungarian_maximum_matching;
use crate::graph_theory::bipartite::bipartite_undirected;
use nalgebra::DMatrix;
use petgraph::algo::maximum_matching;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use rocket::{
        self,
        serde::{json::Json, Deserialize, Serialize},
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

#[derive(Serialize)]
struct ProblemState {
        // u = (u_1, u_2, ..., u_n)
        u: Vec<i32>,

        // v = (v_1, v_2, ..., v_n)
        v: Vec<i32>,

        // c = cost matrix.
        // c_i,j >= 0, u_i + v_j <= c_i,j
        c: DMatrix<i32>,
        // Equality graph (bipartite)
        g: Option<Graph<String, String, Undirected>>,

        // Maximum matching (perfect matching gives the solution)
        m: Option<Vec<(NodeIndex, NodeIndex)>>,
}

#[derive(Serialize)]
pub struct OptimalSolution {
        states: Vec<ProblemState>,
}

impl OptimalSolution {
        pub fn new() -> Self {
                Self { states: Vec::new() }
        }

        pub fn init(&mut self, u_invec: Vec<i32>, v_invec: Vec<i32>, c: DMatrix<i32>) {
                self.states.push(ProblemState {
                        u: u_invec.clone(),
                        v: v_invec.clone(),
                        c: c.clone(),
                        g: None,
                        m: None,
                });
        }

        pub fn find_optimum_cost(&mut self) -> String {
                loop {
                        let u: DMatrix<i32>;
                        let v: DMatrix<i32>;

                        let last_saved_state = self.states.last().unwrap();
                        let first_saved_state = self.states.get(0).unwrap();

                        let prev_c = &first_saved_state.c;
                        let prev_u_invec = &last_saved_state.u;
                        let prev_v_invec = &last_saved_state.v;
                        let prev_g = &last_saved_state.g;
                        // let prev_m = &last_saved_state.m;

                        // println!("{:?}", prev_g);

                        u = generate_u_matrix(prev_c, prev_u_invec);
                        // println!("u = {}", &u);
                        let c_ij_intermediary = prev_c - &u;

                        v = generate_v_matrix(&c_ij_intermediary, prev_v_invec);
                        // println!("v = {}", v);
                        let c_ij = c_ij_intermediary - &v;
                        println!("c_ij = {}", c_ij);

                        let g = generate_equality_graph(&c_ij);

                        println!("g = {:?}", g);
                        let nrows = c_ij.nrows();
                        let ncols = c_ij.ncols();

                        let g_vertex_u_v_sets = bipartite_undirected(&g).unwrap();
                        // println!("nodes_u = {:?}", g_vertex_u_v_sets.nodes_u);
                        // println!("nodes_v = {:?}", g_vertex_u_v_sets.nodes_v);
                        let max_matching = maximum_matching(&g);
                        let mut matched_edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();

                        for edge in max_matching.edges() {
                                // println!("m edge = {:?}", edge);
                                matched_edges.insert(edge);
                        }

                        // let covered_nodes: HashSet<NodeIndex> = max_matching.nodes().into_iter().collect();
                        // println!("covered_nodes = {:?}", covered_nodes);

                        // ###########################
                        // check is_perfect is working properly
                        let perfect = max_matching.is_perfect();
                        // println!("perfect = {:?}", &perfect);

                        if perfect == true {
                                let mut cost = 0;
                                for edge in max_matching.edges() {
                                        println!("edge = {:?}", &edge);
                                        let node1 = edge.0;
                                        let node2 = edge.1;
                                        let node1_index = node1.index();
                                        let node2_index = node2.index() - ncols;
                                        if let Some(elem) = prev_c.get((node1_index % nrows) + (node2_index * ncols)) {
                                                cost = cost + elem;
                                        }
                                }
                                // the matching is optimal perfect matching if matching's cost equals ∑u + ∑v
                                println!("cost = {:?}", cost);
                                let u_sum: i32 = prev_u_invec.iter().sum();
                                let v_sum: i32 = prev_v_invec.iter().sum();
                                println!("uv_sum = {:?}", u_sum + v_sum);

                                let buf = Vec::new();
                                let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
                                let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
                                self.states.serialize(&mut ser).unwrap();
                                return String::from_utf8(ser.into_inner()).unwrap();
                        }

                        let mut node_index_weight_map: HashMap<usize, String> = HashMap::new();

                        for node in g.node_indices() {
                                node_index_weight_map.insert(node.index(), String::from(g.node_weight(node).unwrap()));
                        }

                        if let Some(hungarian_output) = hungarian_maximum_matching(&g, node_index_weight_map, matched_edges, &g_vertex_u_v_sets) {
                                let min_cover = hungarian_output.0;
                                // println!("min_cover = {:?}", min_cover);
                                // max_matching = hungarian_output.1;

                                // min_cover has the set of colored vertices from hungarian algm output. We need to find the
                                // row index and column index of c_ij from the node indices. If the node is a member of the set U,
                                // then node indices suffice as row index. If the node is a member of the set V, then we
                                // need to subtract `ncols` from node index. Then we choose all the nodes in set V which are
                                // not in the min_cover to get the correct column index.
                                // we choose ϵ to be the minimum of c_ij with i and j calculated from node indices as above.
                                let mut min_cover_row_indices: Vec<usize> = g_vertex_u_v_sets.nodes_u.iter().map(|x| x.index()).collect();
                                let mut min_cover_red_indices: Vec<usize> = Vec::new();
                                let mut min_cover_col_indices: Vec<usize> = g_vertex_u_v_sets.nodes_v.iter().map(|x| (x.index() - ncols)).collect();

                                // min_cover contains uncolored nodes from U and colored nodes from V. We need colored node indices from
                                // set U  and uncolored node indices from V to calculate epsilon.
                                for node in &min_cover {
                                        if g_vertex_u_v_sets.nodes_u.contains(&node) {
                                                // 'min_cover_row_indices' has all the node indices represnting each row of
                                                // the c_ij matrix. We remove the elements corresponding to the uncolored vertices
                                                // in min_cover
                                                min_cover_row_indices.retain(|value| *value != node.index());
                                        } else {
                                                // 'min_cover_col_indices' has all the node indices represnting each column of
                                                // the c_ij matrix. We remove the elements corresponding to the red color vertices
                                                // in our min_cover
                                                min_cover_col_indices.retain(|value| *value != (node.index() - ncols));
                                                min_cover_red_indices.push(node.index() - ncols);
                                        }
                                }

                                let min_cover_blue_indices = min_cover_row_indices.clone();

                                // println!("min_cover_row_indices = {:?}", min_cover_row_indices);
                                // println!("min_cover_col_indices = {:?}", min_cover_col_indices);
                                // println!("min_cover_red_indices = {:?}", min_cover_red_indices);

                                //  ϵ calculation
                                let mut min_cover_matrix_elements: Vec<i32> = Vec::new();
                                for i in &min_cover_row_indices {
                                        for j in &min_cover_col_indices {
                                                if let Some(elem) = c_ij.get((i % nrows) + (j * ncols)) {
                                                        min_cover_matrix_elements.push(*elem);
                                                }
                                        }
                                }

                                // println!("min_cover_matrix_elements = {:?}", min_cover_matrix_elements);
                                let epsilon = min_cover_matrix_elements.iter().min().unwrap();
                                // println!("epsilon = {:?}", epsilon);
                                //
                                // increase u vec elements and decrese v vector elements by epsilon
                                //
                                let u_invec_col = u.column(0);
                                let mut u_invec = u_invec_col.into_iter().map(|x| *x).collect::<Vec<_>>();

                                let v_invec_col = v.row(0);
                                let mut v_invec = v_invec_col.into_iter().map(|x| *x).collect::<Vec<_>>();

                                for (i, x) in u_invec.iter_mut().enumerate() {
                                        if min_cover_blue_indices.contains(&i) {
                                                *x = *x + epsilon;
                                        }
                                }

                                for (i, x) in v_invec.iter_mut().enumerate() {
                                        if min_cover_red_indices.contains(&i) {
                                                *x = *x - epsilon;
                                        }
                                }

                                // println!("u_invec = {:?}", &u_invec);
                                // println!("v_invec = {:?}", &v_invec);
                                let new_state = ProblemState {
                                        u: u_invec,
                                        v: v_invec,
                                        c: c_ij,
                                        g: Some(g),
                                        m: None,
                                };

                                self.states.push(new_state);
                        }
                }
        }
}

// impl Serialize for OptimalSolution {
//         fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//                 S: Serializer,
//         {
//                 for state in self.states {
//                 }
//         }
// }

fn generate_u_matrix(c: &DMatrix<i32>, u: &Vec<i32>) -> DMatrix<i32> {
        let nrows = c.nrows();
        let ncols = c.ncols();
        let mut u = u.clone();

        if u.len() == 0 {
                for row in c.row_iter() {
                        u.push(row.min());
                }
        }

        let frequencies = vec![ncols; ncols];
        // u_i to be the smallest entry in row i
        // below expression will convert 1-dim u into nrows x ncols sized matrix with the first
        // element of each row repeating in each colums. For ex. if 3 is element at (0,0), then (0,1)
        // (0,2) and (0,3) will be 3.
        let u_modifier_vals = u
                .into_iter()
                .zip(frequencies.into_iter())
                .map(|(n, t)| iter::repeat(n).take(t))
                .fold(Vec::new(), |mut acc, x| {
                        acc.extend(x);
                        acc
                });
        DMatrix::from_row_slice(nrows, ncols, &u_modifier_vals[..])
}

fn generate_v_matrix(c: &DMatrix<i32>, v: &Vec<i32>) -> DMatrix<i32> {
        let nrows = c.nrows();
        let ncols = c.ncols();
        let mut v = v.clone();

        if v.len() == 0 {
                for column in c.column_iter() {
                        v.push(column.min());
                }
        }

        let v_modifier_vals: Vec<_> = v.into_iter().cycle().take(ncols * nrows).collect();

        DMatrix::from_row_slice(nrows, ncols, &v_modifier_vals[..])
}

fn generate_equality_graph(c_ij: &DMatrix<i32>) -> Graph<String, String, petgraph::Undirected> {
        let nrows = c_ij.nrows();
        let ncols = c_ij.ncols();

        let mut g: Graph<String, String, petgraph::Undirected> = Graph::new_undirected();
        //let mut g: UnGraph<String, String> = UnGraph::new_undirected();

        for i in 0..(nrows as i32) {
                let weight = "s".to_string() + &(i + 1).to_string();
                g.add_node(weight);
        }

        for j in 0..(ncols as i32) {
                let weight = "s".to_string() + &(j + 1).to_string();
                g.add_node(weight);
        }

        // let is_bipartite = is_bipartite_undirected(&g, NodeIndex::new(0));
        // iterate the elements of the cost matrix in a column major way and calculate the row index
        // and column index backward from the element position ('i' below). Since the sets U and V of the bipartite
        // graph cannot have the same index numbers, we add numger of columns to column index to get the node
        // index of the second node of the edge in the graph
        for (i, element) in c_ij.iter().enumerate() {
                match *element {
                        0 => {
                                let row_index = i % nrows;
                                let col_index = i / nrows;

                                g.add_edge(
                                        NodeIndex::new(row_index),
                                        NodeIndex::new(ncols + col_index),
                                        format!("{} -> {}", row_index, ncols + col_index),
                                );
                        }
                        _ => (),
                }
        }

        g
}
