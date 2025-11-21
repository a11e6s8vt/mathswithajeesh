// This function is adapted from the `petgraph` crate (version 0.6.0).
// Original source:
// https://github.com/petgraph/petgraph/blob/master/src/graph/whatever_path.rs
//
// License: MIT OR Apache-2.0 (see /licenses folder in this repository).
//
// Modifications in this version:
// - The original `is_bipartite_undirected` assumed a connected graph.
// - Adjusted the logic to correctly handle disconnected bipartite graphs.
// - Modified selected private methods to make them public for integration.
// - Additional minor changes for compatibility with this project.
//
// Copyright (c) 2014-2023
// The petgraph developers.
//
extern crate petgraph;

use petgraph::visit::{GraphRef, IntoNeighbors, IntoNodeIdentifiers, VisitMap, Visitable};

pub struct BipartiteGraph<N> {
        pub nodes_u: Vec<N>,
        pub nodes_v: Vec<N>,
}

/// Return `true` if the graph is bipartite.
///
/// A graph is bipartite if it's nodes can be divided into two disjoint and indepedent
/// sets `U` and `V` such that every edge connects `U` to one in `V`.
/// This function implements 2-coloring algorithm based on BFS.
///
/// The input graph is always treated as undirected.
pub fn is_bipartite_undirected<G, N, VM>(g: G) -> bool
where
        G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N> + IntoNodeIdentifiers<NodeId = N>,
        N: Copy + PartialEq + std::fmt::Debug,
        VM: VisitMap<N>,
{
        let node_ids: Vec<N> = g.node_identifiers().map(|id| id).collect();

        let mut red = g.visit_map();
        let mut blue = g.visit_map();

        for node in node_ids {
                for neighbor in g.neighbors(node) {
                        let is_blue = blue.is_visited(&node);
                        let is_red = red.is_visited(&node);
                        let is_neighbor_blue = blue.is_visited(&neighbor);
                        let is_neighbor_red = red.is_visited(&neighbor);

                        match (is_blue, is_red, is_neighbor_blue, is_neighbor_red) {
                                (false, false, false, false) => {
                                        blue.visit(node);
                                        red.visit(neighbor);
                                }
                                (false, false, false, true) => {
                                        blue.visit(node);
                                }
                                (false, false, true, false) => {
                                        red.visit(node);
                                }
                                (true, false, false, false) => {
                                        red.visit(neighbor);
                                }
                                (true, false, false, true) => {
                                        continue;
                                }
                                (false, true, true, false) => {
                                        continue;
                                }
                                (_, _, _, _) => return false,
                        }
                }
        }

        true
}

pub fn bipartite_undirected<G, N, VM>(g: G) -> Option<BipartiteGraph<N>>
where
        G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N> + IntoNodeIdentifiers<NodeId = N>,
        N: Copy + PartialEq + std::fmt::Debug,
        VM: VisitMap<N>,
{
        let mut nodes_u: Vec<N> = Vec::new();
        let mut nodes_v: Vec<N> = Vec::new();
        let r_nodes: Vec<N> = g.node_identifiers().map(|x| x).collect();
        // println!("r_nodes = {:?}", r_nodes);

        let mut blue = g.visit_map();
        let mut red = g.visit_map();

        for node in r_nodes {
                for neighbor in g.neighbors(node) {
                        let is_blue = blue.is_visited(&node);
                        let is_red = red.is_visited(&node);
                        let is_neighbor_blue = blue.is_visited(&neighbor);
                        let is_neighbor_red = red.is_visited(&neighbor);
                        match (is_blue, is_red, is_neighbor_blue, is_neighbor_red) {
                                (false, false, false, false) => {
                                        blue.visit(node);
                                        red.visit(neighbor);
                                }
                                (true, false, false, false) => {
                                        red.visit(neighbor);
                                }
                                (false, false, false, true) => {
                                        blue.visit(node);
                                }
                                (_, _, _, _) => {
                                        continue;
                                }
                        }
                }
        }

        for node in g.node_identifiers() {
                if blue.is_visited(&node) {
                        nodes_u.push(node);
                } else if red.is_visited(&node) {
                        nodes_v.push(node);
                }
        }

        Some(BipartiteGraph { nodes_u, nodes_v })
}
