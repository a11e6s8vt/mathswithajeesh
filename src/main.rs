#![allow(dead_code)]
use maths::comb_optimization::OptimalSolution;
use rocket::serde::json::Json;
use web::inputs::{parse, InputAssPblm};

mod web;

// fn main() {
//         match inputs::parse::<u32>() {
//                 Ok((u_invec, v_invec, c)) => {
//                         let mut ass_pblm_soln = OptimalSolution::new();
//                         ass_pblm_soln.init(u_invec, v_invec, c);
//                         ass_pblm_soln.find_optimum_cost();
//                 }
//                 Err(err) => println!("{:?}", err),
//         }
// }
#[macro_use]
extern crate rocket;

#[post("/", format = "json", data = "<ass_pblm_input>")]
fn solve_assignment_problem(ass_pblm_input: Json<InputAssPblm>) -> String {
        match parse::<i32>(ass_pblm_input) {
                Ok((u_invec, v_invec, c)) => {
                        let mut ass_pblm_soln = OptimalSolution::new();
                        ass_pblm_soln.init(u_invec, v_invec, c);
                        let ass_pblm_soln = ass_pblm_soln.find_optimum_cost();
                        return ass_pblm_soln;
                }
                Err(err) => println!("{:?}", err),
        }
       "".to_string()
}

use rocket::response::stream::{Event, EventStream};
use rocket::futures::stream;

#[get("/events")]
fn events() -> EventStream![] {
        let event = Event::data("Hello, SSE!")
                .with_comment("just a hello message")
                .event("hello")
                .id("1");
        let raw = stream::iter(vec![Event::data("a"), Event::data("b"), event]);
        let stream = EventStream::from(raw);
        stream
}

#[rocket::main]
async fn main() {
        if let Err(err) = rocket::build()
                .mount("/co/assignment_problem/solve", routes![solve_assignment_problem])
                .mount("/", routes![events])
        .launch().await {
                println!("Rocket Rust couldn't take off successfully!");
                drop(err); // Drop initiates Rocket-formatted panic
        }
}
