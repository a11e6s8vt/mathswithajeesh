#![allow(dead_code)]

use nalgebra::{DMatrix, Scalar};
use rocket::serde::json::Json;
use serde::Deserialize;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct InputAssPblm {
        u: Option<Vec<i32>>,
        v: Option<Vec<i32>>,
        c: Vec<Vec<i32>>,
}

pub fn parse<N>(ass_pblm_input: Json<InputAssPblm>) -> Result<(Vec<i32>, Vec<i32>, DMatrix<i32>), Box<dyn Error>>
where
        N: FromStr + Scalar,
        N::Err: Error,
{
        let mut u: Vec<i32> = Vec::new();
        let mut v: Vec<i32> = Vec::new();
        let c: DMatrix<i32>;

        if let Some(u_in) = &ass_pblm_input.u {
                u = u_in.clone();
        }

        if let Some(v_in) = &ass_pblm_input.v {
                v = v_in.clone();
        }

        let c_in = &ass_pblm_input.c;
        c = get_cost_matrix(c_in.clone());
        
        Ok((u, v, c))
}

fn get_cost_matrix(c_inmatrix: Vec<Vec<i32>>) -> DMatrix<i32> {
        let mut c = Vec::new();
        let rows;
        let cols;

        rows = c_inmatrix.len();

        for mut row in c_inmatrix {
                c.append(&mut row);
        }

        // The number of items divided by the number of rows equals the
        // number of columns.
        cols = c.len() / rows;
        DMatrix::from_row_slice(rows, cols, &c[..])
}
