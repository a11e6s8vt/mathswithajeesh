pub use maths::comb_optimization::hungarian::hungarian_maximum_matching;
pub use maths::graph_theory::bipartite::bipartite_undirected;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::HashSet;

#[test]
fn test_hungarian_maximum_matching() {
        {
                let mut graph: Graph<String, String, Undirected> = Graph::new_undirected();
                let a = graph.add_node("s1".to_string());
                let b = graph.add_node("s2".to_string());
                let c = graph.add_node("s3".to_string());
                let d = graph.add_node("s4".to_string());
                let e = graph.add_node("s5".to_string());

                let f = graph.add_node("t1".to_string());
                let g = graph.add_node("t2".to_string());
                let h = graph.add_node("t3".to_string());
                let i = graph.add_node("t4".to_string());
                let j = graph.add_node("t5".to_string());

                graph.add_edge(a, g, "s1-t2".to_string());
                graph.add_edge(a, h, "s1-t3".to_string());
                graph.add_edge(b, f, "s2-t1".to_string());
                graph.add_edge(b, g, "s2-t2".to_string());
                graph.add_edge(b, i, "s2-t4".to_string());
                graph.add_edge(b, j, "s2-t5".to_string());
                graph.add_edge(c, g, "s3-t2".to_string());
                graph.add_edge(c, h, "s3-t3".to_string());
                graph.add_edge(d, g, "s4-t2".to_string());
                graph.add_edge(d, h, "s4-t3".to_string());
                graph.add_edge(e, i, "s5-t4".to_string());
                graph.add_edge(e, j, "s5-t5".to_string());

                let mut input_max_matching: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();
                input_max_matching.insert((b, g));
                input_max_matching.insert((c, h));
                input_max_matching.insert((e, j));
                let mut node_index_weight_map: HashMap<usize, String> = HashMap::new();

                for node in graph.node_indices() {
                        let weight = String::from(graph.node_weight(node).unwrap());
                        node_index_weight_map.insert(node.index(), weight.clone());
                }

                let g_vertex_u_v_sets = bipartite_undirected(&graph).unwrap();
                let hungarian_output = hungarian_maximum_matching(
                        &graph,
                        node_index_weight_map,
                        input_max_matching,
                        &g_vertex_u_v_sets,
                )
                .unwrap();
                let min_cover_test = vec![NodeIndex::new(1), NodeIndex::new(4), NodeIndex::new(6), NodeIndex::new(7)];
                let min_cover = hungarian_output.0;
                let max_matching = hungarian_output.1;
                println!("{:?}", min_cover);
                assert_eq!(min_cover, min_cover_test);
                println!("{:?}", max_matching);
        }
        {
                let mut graph: Graph<String, String, Undirected> = Graph::new_undirected();
                let a = graph.add_node("s1".to_string()); // 0
                let b = graph.add_node("s2".to_string()); // 1
                let c = graph.add_node("s3".to_string()); // 2
                let d = graph.add_node("s4".to_string()); // 3
                let e = graph.add_node("s5".to_string()); // 4

                let f = graph.add_node("t1".to_string()); // 5
                let g = graph.add_node("t2".to_string()); // 6
                let h = graph.add_node("t3".to_string()); // 7
                let i = graph.add_node("t4".to_string()); // 8
                let j = graph.add_node("t5".to_string()); // 9

                graph.add_edge(a, h, "s1-t3".to_string()); // (0, 7)
                graph.add_edge(a, i, "s1-t4".to_string()); // (0, 8)
                graph.add_edge(b, f, "s2-t1".to_string());
                graph.add_edge(b, g, "s2-t2".to_string());
                graph.add_edge(b, h, "s2-t3".to_string());
                graph.add_edge(b, j, "s2-t5".to_string());
                graph.add_edge(c, h, "s3-t3".to_string());
                graph.add_edge(d, f, "s4-t1".to_string());
                graph.add_edge(d, g, "s4-t2".to_string());
                graph.add_edge(d, j, "s4-t5".to_string());
                graph.add_edge(e, h, "s5-t3".to_string());
                graph.add_edge(e, i, "s5-t4".to_string());

                let mut input_max_matching: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();
                input_max_matching.insert((b, h));
                input_max_matching.insert((e, i));
                let mut node_index_weight_map: HashMap<usize, String> = HashMap::new();

                for node in graph.node_indices() {
                        let weight = String::from(graph.node_weight(node).unwrap());
                        node_index_weight_map.insert(node.index(), weight.clone());
                }

                let g_vertex_u_v_sets = bipartite_undirected(&graph).unwrap();
                let hungarian_output = hungarian_maximum_matching(
                        &graph,
                        node_index_weight_map,
                        input_max_matching,
                        &g_vertex_u_v_sets,
                )
                .unwrap();
                let min_cover_test = vec![NodeIndex::new(1), NodeIndex::new(3), NodeIndex::new(7), NodeIndex::new(8)];
                let min_cover = hungarian_output.0;
                let max_matching = hungarian_output.1;
                println!("{:?}", min_cover);
                assert_eq!(min_cover, min_cover_test);
                println!("{:?}", max_matching);
        }
        {
                let mut graph: Graph<String, String, Undirected> = Graph::new_undirected();
                let a = graph.add_node("s1".to_string()); // 0
                let b = graph.add_node("s2".to_string()); // 1
                let c = graph.add_node("s3".to_string()); // 2
                let d = graph.add_node("s4".to_string()); // 3

                let e = graph.add_node("t1".to_string());
                let f = graph.add_node("t2".to_string());
                let g = graph.add_node("t3".to_string());
                let h = graph.add_node("t4".to_string());

                graph.add_edge(a, f, "s1-t2".to_string());
                graph.add_edge(b, g, "s2-t3".to_string());
                graph.add_edge(b, h, "s2-t4".to_string());
                graph.add_edge(c, f, "s3-t2".to_string());
                graph.add_edge(d, e, "s4-t1".to_string());
                graph.add_edge(d, f, "s4-t2".to_string());

                let mut input_max_matching: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();
                input_max_matching.insert((a, f));
                input_max_matching.insert((d, e));
                let mut node_index_weight_map: HashMap<usize, String> = HashMap::new();

                for node in graph.node_indices() {
                        let weight = String::from(graph.node_weight(node).unwrap());
                        node_index_weight_map.insert(node.index(), weight.clone());
                }

                let g_vertex_u_v_sets = bipartite_undirected(&graph).unwrap();
                let hungarian_output = hungarian_maximum_matching(
                        &graph,
                        node_index_weight_map,
                        input_max_matching,
                        &g_vertex_u_v_sets,
                )
                .unwrap();
                let min_cover_test = vec![NodeIndex::new(1), NodeIndex::new(3), NodeIndex::new(5)];
                let min_cover = hungarian_output.0;
                let max_matching = hungarian_output.1;
                println!("{:?}", min_cover);
                assert_eq!(min_cover, min_cover_test);
                println!("{:?}", max_matching);
        }
}
