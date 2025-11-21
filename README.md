# MathsWithAjeesh

> **Note:** This project is currently a proof of concept.  
> It will grow into a fully fledged mathematical and algorithmic toolkit over time.

A Rust toolkit **and learning platform** for combinatorics, graph algorithms, combinatorial optimisation, and number theory. It's built for **learning, deep understanding, research, and practical computation**. Often, as learners and researchers, we get stuck on computations involving large numbers or subtle algorithmic steps and wish we had a tool to verify them. Many existing tools are not free or not transparent. MathsWithAjeesh is an attempt to fill that gap with an open, accessible, and mathematically precise platform.

MathsWithAjeesh combines two worlds:

1. **Algorithmic clarity**: Focus is on correctness and accuracy of implementation.
2. **Rust for mathematical computing**: This project is a platform for learning and improving mathematical computing in Rust. It also provides a tool for exploring advanced mathematical concepts.

The project is designed for students, engineers, and researchers who want to _learn by building_.

---

## 🎓 Project Philosophy — _Code as a Learning Platform_

MathsWithAjeesh is based on a simple idea:

> **When you understand the mathematics deeply, it's easy to express it in code.  
> And when you write the code well, the mathematics becomes clearer.**

The goal of this repository is two-fold - it can act as a **guided mathematical notebook** and an **algorithmic learning platform** built around the following goals:

### **1. Learn by Building**

Computational implementation of an algorithm, with a focus on correctness, helps deepen understanding of the underlying mathematics. The programmatic challenges teach the limitations of computing, which we do not face with paper and pencil. At the same time, it helps iterate over large matrices, graphs, or datasets that would be impractical to handle by hand.

The long-term goal is to develop everything from first principles. To bootstrap the project and start working on the core algorithms, such as Hungarian assignment and bipartite matching, we have used the `petgraph` crate, with some modifications to address edge cases not handled correctly in crates (e.g., disconnected graphs).

### **2. Mathematics + Code Together**

Each module includes:

- The mathematical formulation
- Worked examples
- References to classical papers or books

The long-term aim: **a unified, mathematical computation toolkit in Rust**.

### **3. Teaching and research Friendly**

This repository is part of a long-term academic goal. In the future, it may evolve into:

- an mdBook
- lecture notes
- a series of computational mathematics tutorials
- a toolkit used in mathematical research

## 📦 Current Modules

Although the main public interface is currently an HTTP API, the core algorithms live in internal Rust modules. The long-term plan is to expose these as a stable library API.

### 🔹 Graph Theory

#### **1. Bipartite Matching (`maths::graph_theory::bipartite`)**

- Modified the function in `petgraph` crate to consider disconnected bipartite graphs using BFS

### 🔹 Combinatorial Optimisation

#### **1. Hungarian Algorithm (`maths::comb_optimization::hungarian`)**

- Full Kuhn–Munkres algorithm implementation.
- Finds the `maximum-matching` in a graph.

#### **2. Assignment Problem (`maths::comb_optimization::assignment_problem`)**

- Implementation of the classical assignment problem.
- Supports rectangular cost matrices.  
  - Returns optimal assignment with minimum cost.

### 🔹 (Planned) Number Theory and Other Modules

The project will gradually add modules for:

- number theory (primality testing, modular arithmetic, Pollard–Rho, etc.)
- optimisation and flows
- probabilistic models (HMMs, Gibbs sampling, ZIG models)

These will live under modules such as `maths::number_theory::…` and
`maths::stochastics::…` as they are implemented.

## 🧪 How to Use It

At the moment, the main way to use MathsWithAjeesh is via an HTTP API implemented with the Rocket web framework. Internally, the web server calls the assignment problem modules (e.g. `maths::comb_optimization::assignment_problem`), and a direct Rust library API will be exposed once the interfaces stabilise.

### 🚀 Running the Server

Start the backend with:

```bash
cargo run
```

Rocket will start a server at:

```bash
http://127.0.0.1:8000
```

### 📍 Endpoint: Solve Assignment Problem

**Method:** POST
**URL:** /co/assignment_problem/solve

This endpoint computes an optimal assignment using the Hungarian (Kuhn–Munkres) algorithm from `maths::graph_theory::hungarian`.

### 📨 Input Format (JSON)

```json
{
  "u": [7, 8, 6, 9, 8],
  "v": [4, 3, 6, 5, 4],
  "c": [
    [13, 12, 13, 12, 11],
    [12, 11, 15, 15, 16],
    [10, 13, 18, 16, 13],
    [13, 15, 22, 16, 17],
    [12, 15, 19, 16, 18]
  ]
}
```

**Field descriptions:**
u – values associated with the first set (rows)
v – values associated with the second set (columns)
c – cost matrix (rows × columns)

### 🧪 Example curl Command

```bash
curl -H "Content-Type: application/json" \
  --data '{"u":[7,8,6,9,8],
           "v":[4,3,6,5,4],
           "c":[[13,12,13,12,11],
                [12,11,15,15,16],
                [10,13,18,16,13],
                [13,15,22,16,17],
                [12,15,19,16,18]]}' \
  http://127.0.0.1:8000/co/assignment_problem/solve
```

### 📤 Example Response

The server returns a list of computational snapshots representing stages of the Hungarian algorithm. Each snapshot includes:

- updated vectors u and v
- a flattened cost matrix c with dimensions
- a graph structure g (or null)
- a placeholder m for the final matching

```json
[
  {
    "u": [7, 8, 6, 9, 8],
    "v": [4, 3, 6, 5, 4],
    "c": [
      [
        13, 12, 10, 13, 12, 12, 11, 13, 15, 15, 13, 15, 18, 22, 19, 12, 15, 16,
        16, 16, 11, 16, 13, 17, 18
      ],
      5,
      5
    ],
    "g": null,
    "m": null
  },
  {
    "u": [7, 8, 8, 11, 10],
    "v": [2, 3, 6, 5, 4],
    "c": [
      [
        2, 0, 0, 0, 0, 2, 0, 4, 3, 4, 0, 1, 6, 7, 5, 0, 2, 5, 2, 3, 0, 4, 3, 4,
        6
      ],
      5,
      5
    ],
    "g": {
      "nodes": ["s1", "s2", "s3", "s4", "s5", "s1", "s2", "s3", "s4", "s5"],
      "node_holes": [],
      "edge_property": "undirected",
      "edges": [
        [1, 5, "1 -> 5"],
        [2, 5, "2 -> 5"],
        [3, 5, "3 -> 5"],
        [4, 5, "4 -> 5"],
        [1, 6, "1 -> 6"],
        [0, 7, "0 -> 7"],
        [0, 8, "0 -> 8"],
        [0, 9, "0 -> 9"]
      ]
    },
    "m": null
  },
  {
    "u": [7, 8, 9, 11, 11],
    "v": [1, 3, 6, 5, 4],
    "c": [
      [
        4, 2, 0, 0, 0, 2, 0, 2, 1, 2, 0, 1, 4, 5, 3, 0, 2, 3, 0, 1, 0, 4, 1, 2,
        4
      ],
      5,
      5
    ],
    "g": {
      "nodes": ["s1", "s2", "s3", "s4", "s5", "s1", "s2", "s3", "s4", "s5"],
      "node_holes": [],
      "edge_property": "undirected",
      "edges": [
        [2, 5, "2 -> 5"],
        [3, 5, "3 -> 5"],
        [4, 5, "4 -> 5"],
        [1, 6, "1 -> 6"],
        [0, 7, "0 -> 7"],
        [0, 8, "0 -> 8"],
        [3, 8, "3 -> 8"],
        [0, 9, "0 -> 9"]
      ]
    },
    "m": null
  }
]
```

### 🔍 Interpreting the Response

- Each array element is a phase of the Hungarian algorithm.
- The cost matrix c is stored as a flattened vector followed by rows and columns.
- g shows the bipartite graph structure at that stage.
- m will later contain the final assignment and cost.

### 📦 Planned: Direct Rust Library API

Internally, the Rocket server calls `maths::comb_optimization::assigment_problem` which in turn calls:

- `maths::graph_theory::hungarian`
- `maths::graph_theory::bipartite_matching`

A future version will expose a direct Rust API such as:

```rust
use mathswithajeesh::maths::graph_theory::hungarian::solve_assignment;
```

For now, the HTTP API is the official interface.

## 📜 License

This project is licensed under the MIT License.  
See the `LICENSE` file for the full text.

### Third-Party Code Attribution

This project includes code adapted from the `petgraph` crate (version 0.6.0),  
which is dual-licensed under MIT OR Apache-2.0.

The original license texts are included in the `licenses/` directory.
