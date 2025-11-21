use serde::Deserialize;
use std::str::FromStr;
use std::error::Error;
use nalgebra::{DMatrix, Scalar};
use std::io::{self, BufRead};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UInVec(Option<Vec<i32>>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VInVec(Option<Vec<i32>>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CInMatrix(Vec<Vec<i32>>);

// echo -e "{\"u\":[2,4,3,2],\"v\":[0,0,0,1],\"c\":[[5,2,3,4],[7,8,4,5],[6,3,5,6],[2,2,3,5]]}" | cargo run --
/// Consumes a `BufRead` of line and json string of u, v and cost matrix, and
/// produces either a `DMatrix` of the three or an error.
pub fn parse<N>() -> Result<(Vec<i32>, Vec<i32>, DMatrix<i32>), Box<dyn Error>>
where
    N: FromStr + Scalar,
    N::Err: Error,
{
    let mut u: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    let mut c: DMatrix<i32> = DMatrix::zeros(0, 0);
    let stdin = io::stdin();
    let buffered_stdin = stdin.lock();

    // for each line in the input,
    for line in buffered_stdin.lines() {
        match line {
            Ok(buffered_stdin) => {
                let input_json: serde_json::Value =
                    serde_json::from_str(&buffered_stdin).expect("JSON was not well-formatted");

                u = get_u_vector(&input_json);
                v = get_v_vector(&input_json);
                c = get_cost_matrix(&input_json);
            }
            Err(err) => println!("Input failed!!! {}", err),
        }
    }

    Ok((u, v, c))
}

fn get_u_vector(input_json: &serde_json::Value) -> Vec<i32> {
    let mut u = Vec::new();

    if let Some(u_json_arr) = input_json.get("u") {
        let u_invec: UInVec = serde_json::from_value(u_json_arr.clone()).unwrap();
        if let Ok(item) = u_invec.0.ok_or("None") {
            u = item.to_vec();
        }
    }
    u
    //DMatrix::from_row_slice(u.len(), 1, &u[..])
}

fn get_v_vector(input_json: &serde_json::Value) -> Vec<i32> {
    let mut v = Vec::new();

    if let Some(v_json_arr) = input_json.get("v") {
        let v_invec: VInVec = serde_json::from_value(v_json_arr.clone()).unwrap();
        if let Ok(item) = v_invec.0.ok_or("None") {
            v = item.to_vec();
        }
    }
    v
    //DMatrix::from_row_slice(v.len(), 1, &v[..])
}

fn get_cost_matrix(input_json: &serde_json::Value) -> DMatrix<i32> {
    let mut c = Vec::new();
    let mut rows = 0;
    let mut cols = 0;

    if let Some(c_json_arr) = input_json.get("c") {
        let c_inmatrix: CInMatrix = serde_json::from_value(c_json_arr.clone()).unwrap();
        
        rows = c_inmatrix.0.len();

        for mut row in c_inmatrix.0 {
            c.append(&mut row);
        }

        // The number of items divided by the number of rows equals the
        // number of columns.
        cols = c.len() / rows;
    }

    DMatrix::from_row_slice(rows, cols, &c[..])
}

