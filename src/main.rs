//TODO:
// return a count on nbr or recursive calls from is dead, to find the longest of the bad.
// this number could be score value, say if its 100/200/300 instead of 1/2/3 then this could add to
// ie. 215 
// Possible collision doesn't account for smaller snake, should it in path finding? 
// should pathfinding really regard possible move?

#![feature(proc_macro_hygiene, decl_macro)]

// Modules
#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod responses;
#[cfg(test)]
mod test;

// External crates
#[macro_use]
extern crate rocket;
extern crate rand;
extern crate rocket_contrib;

// Uses
use rocket::http::Status;
use rocket_contrib::json::Json;

const MY_SNAKE_NAME: &str = "ChoffesBattleSnakeV1";
const DEBUG: bool = true;

#[get("/")]
fn index() -> Json<responses::Info> {
    Json(responses::Info {
        apiversion: "1".to_string(),
        author: None,
        color: Some("#b7410e".to_string()),
        head: None,
        tail: None,
        version: Some("0".to_string()),
    })
}

#[post("/start")]
fn start() -> Status {
    Status::Ok
}

fn is_dead_end(
    req: &requests::Turn,
    obs: &Vec<requests::Point>,
    visited: &mut Vec<requests::Point>,
    p: &requests::Point,
    depth: &mut i32,
) -> bool {
    if p.x < 0 || p.x >= req.board.width {
        return true;
    } else if p.y < 0 || p.y >= req.board.height {
        return true;
    }
    if obs.contains(&p) {
        return true;
    }
    let man_dist = (p.x - req.you.head.x).abs() + (p.y - req.you.head.y).abs();
    let limit = std::cmp::min((req.you.length as i32 / 2) + 1, req.board.width);
    if man_dist > limit {
        return false;
    }

    if visited.contains(&p) {
        return true;
    }
    visited.push(p.clone());
    
    let possible_moves = [
        requests::Point { x: -1, y: 0 },
        requests::Point { x: 0, y: 1 },
        requests::Point { x: 0, y: -1 },
        requests::Point { x: 1, y: 0 },
    ];

    *depth += 1;
    let res = possible_moves
        .iter()
        .map(|point| is_dead_end(req, obs, visited, &(*p + *point), depth))
        .all(|x| x);
    res
}

fn move_score(req: &requests::Turn, p: requests::Point) -> i32 {
    let my_id = req.you.id.clone();
    let possible_moves = [
        requests::Point { x: -1, y: 0 },
        requests::Point { x: 0, y: 1 },
        requests::Point { x: 0, y: -1 },
        requests::Point { x: 1, y: 0 },
    ];

    if p.x < 0 || p.x >= req.board.width {
        return 0;
    } else if p.y < 0 || p.y >= req.board.height {
        return 0;
    }

    let obsticle: Vec<requests::Point> = req
        .board
        .snakes
        .iter()
        .flat_map(|snake| {
            let mut no_tail = snake.body.clone();
            no_tail.pop();
            no_tail
        })
        .collect();
    let possible_collisions: Vec<requests::Point> = req
        .board
        .snakes
        .iter()
        .flat_map(|snake| {
            let mut possible_next_move = Vec::new();
            if snake.id != my_id && snake.length >= req.you.length {
                possible_next_move = possible_moves
                    .clone()
                    .iter()
                    .map(|point| *point + snake.head)
                    .collect();
            }
            possible_next_move
        })
        .collect();

    if obsticle.contains(&p) {
        return 0;
    }
    if possible_collisions.contains(&p) {
        return 100;
    }
    let mut visited = Vec::new();
    let mut obs = obsticle.clone();
    obs.append(&mut possible_collisions.clone());

    let mut length = 0;

    if is_dead_end(req, &obs, &mut visited, &p, &mut length) {
        return 200 + (100 - length);
    }

    let possible_kills: Vec<requests::Point> = req
        .board
        .snakes
        .iter()
        .flat_map(|snake| {
            let mut possible_next_move = Vec::new();
            if snake.id != my_id && snake.length < req.you.length {
                possible_next_move = possible_moves
                    .clone()
                    .iter()
                    .map(|point| *point + snake.head)
                    .collect();
            }
            possible_next_move
        })
        .collect();

    if possible_kills.contains(&p) {
        return 500;
    }

    if req.board.food.contains(&p) {
        return 400;
    }

    300
    // req : Turn { game: Game { id: "5707d540-b1d2-49a9-8700-451d9fe13c9e", timeout: 500 }, turn: 139, board: Board { height: 11, width: 11, food: [Point { x: 8, y: 9 }, Point { x: 1, y: 3 }, Point { x: 9, y: 6 }, Point { x: 7, y: 3 }, Point { x: 5, y: 8 }, Point { x: 4, y: 6 }, Point { x: 10, y: 2 }, Point { x: 5, y: 6 }, Point { x: 4, y: 4 }, Point { x: 7, y: 7 }], snakes: [Snake { id: "gs_M444MRWT4xMXyJHY64HhK7Bf", name: "ChoffesBattleSnakeV1", health: 95, body: [Point { x: 0, y: 3 }, Point { x: 0, y: 2 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 2, y: 3 }, Point { x: 2, y: 4 }, Point { x: 1, y: 4 }, Point { x: 1, y: 5 }, Point { x: 0, y: 5 }, Point { x: 0, y: 6 }, Point { x: 1, y: 6 }, Point { x: 2, y: 6 }, Point { x: 3, y: 6 }, Point { x: 3, y: 7 }], head: Point { x: 0, y: 3 }, length: 16, shout: "", squad: None, latency: "363" }], hazards: [] }, you: Snake { id: "gs_M444MRWT4xMXyJHY64HhK7Bf", name: "ChoffesBattleSnakeV1", health: 95, body: [Point { x: 0, y: 3 }, Point { x: 0, y: 2 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 2, y: 3 }, Point { x: 2, y: 4 }, Point { x: 1, y: 4 }, Point { x: 1, y: 5 }, Point { x: 0, y: 5 }, Point { x: 0, y: 6 }, Point { x: 1, y: 6 }, Point { x: 2, y: 6 }, Point { x: 3, y: 6 }, Point { x: 3, y: 7 }], head: Point { x: 0, y: 3 }, length: 16, shout: "", squad: None, latency: "363" } }
}

fn choose_move(req: requests::Turn) -> responses::Move {
    let neigbours = [
        requests::Point { x: -1, y: 0 },
        requests::Point { x: 0, y: 1 },
        requests::Point { x: 0, y: -1 },
        requests::Point { x: 1, y: 0 },
    ];
    let movements = [
        responses::Movement::Left,
        responses::Movement::Up,
        responses::Movement::Down,
        responses::Movement::Right,
    ];
    let my_pos = req.you.head;
    if DEBUG {
        println!("my_pos {:?}", my_pos);
    }
    let n_score = neigbours
        .iter()
        .map(|n| move_score(&req, *n + my_pos))
        .enumerate();
    let best_score = n_score
        .clone()
        .max_by(|(_, n1), (_, n2)| n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(_, i)| i)
        .unwrap();
    let candidates: Vec<(usize, i32)> = n_score.filter(|(_, v)| v >= &best_score).collect();
    let chosen_index = (rand::random::<f32>() * candidates.len() as f32) as usize;
    let chosen = candidates[chosen_index];

    if DEBUG {
        println!("best {}  with score {}", chosen.0, chosen.1);
        println!("send move {:?}", movements[chosen.0]);
    }
    responses::Move::new(movements[chosen.0])
}

#[post("/move", data = "<req>")]
fn movement(req: Json<requests::Turn>) -> Json<responses::Move> {
    let movement = choose_move(req.into_inner());
    Json(movement)
}

#[post("/end")]
fn end() -> Status {
    Status::Ok
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, start, movement, end])
}

fn main() {
    rocket().launch();
}
