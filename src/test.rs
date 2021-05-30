use super::rocket;
use crate::responses;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn start() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/start")
        .header(ContentType::JSON)
        .body(
            r#"{
                "game": {
                  "id": "game-00fe20da-94ad-11ea-bb37",
                  "ruleset": {
                    "name": "standard",
                    "version": "v.1.2.3"
                  },
                  "timeout": 500
                },
                "turn": 0,
                "board": {
                  "height": 11,
                  "width": 11,
                  "food": [
                    {"x": 5, "y": 5}, 
                    {"x": 9, "y": 0}, 
                    {"x": 2, "y": 6}
                  ],
                  "hazards": [
                    {"x": 3, "y": 2}
                  ],
                  "snakes": [
                    {
                      "id": "snake-508e96ac-94ad-11ea-bb37",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 0, "y": 0}, 
                        {"x": 1, "y": 0}, 
                        {"x": 2, "y": 0}
                      ],
                      "latency": "111",
                      "head": {"x": 0, "y": 0},
                      "length": 3,
                      "shout": "why are we shouting??",
                      "squad": ""
                    }, 
                    {
                      "id": "snake-b67f4906-94ae-11ea-bb37",
                      "name": "Another Snake",
                      "health": 16,
                      "body": [
                        {"x": 5, "y": 4}, 
                        {"x": 5, "y": 3}, 
                        {"x": 6, "y": 3},
                        {"x": 6, "y": 2}
                      ],
                      "latency": "222",
                      "head": {"x": 5, "y": 4},
                      "length": 4,
                      "shout": "I'm not really sure...",
                      "squad": ""
                    }
                  ]
                },
                "you": {
                  "id": "snake-508e96ac-94ad-11ea-bb37",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 0, "y": 0}, 
                    {"x": 1, "y": 0}, 
                    {"x": 2, "y": 0}
                  ],
                  "latency": "111",
                  "head": {"x": 0, "y": 0},
                  "length": 3,
                  "shout": "why are we shouting??",
                  "squad": ""
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn movement() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/move")
        .header(ContentType::JSON)
        .body(
            r#"{
                "game": {
                  "id": "game-00fe20da-94ad-11ea-bb37",
                  "ruleset": {
                    "name": "standard",
                    "version": "v.1.2.3"
                  },
                  "timeout": 500
                },
                "turn": 14,
                "board": {
                  "height": 11,
                  "width": 11,
                  "food": [
                    {"x": 5, "y": 5}, 
                    {"x": 9, "y": 0}, 
                    {"x": 2, "y": 6}
                  ],
                  "hazards": [
                    {"x": 3, "y": 2}
                  ],
                  "snakes": [
                    {
                      "id": "snake-508e96ac-94ad-11ea-bb37",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 0, "y": 0}, 
                        {"x": 1, "y": 0}, 
                        {"x": 2, "y": 0}
                      ],
                      "latency": "111",
                      "head": {"x": 0, "y": 0},
                      "length": 3,
                      "shout": "why are we shouting??",
                      "squad": ""
                    }, 
                    {
                      "id": "snake-b67f4906-94ae-11ea-bb37",
                      "name": "Another Snake",
                      "health": 16,
                      "body": [
                        {"x": 5, "y": 4}, 
                        {"x": 5, "y": 3}, 
                        {"x": 6, "y": 3},
                        {"x": 6, "y": 2}
                      ],
                      "latency": "222",
                      "head": {"x": 5, "y": 4},
                      "length": 4,
                      "shout": "I'm not really sure...",
                      "squad": ""
                    }
                  ]
                },
                "you": {
                  "id": "snake-508e96ac-94ad-11ea-bb37",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 0, "y": 0}, 
                    {"x": 1, "y": 0}, 
                    {"x": 2, "y": 0}
                  ],
                  "latency": "111",
                  "head": {"x": 0, "y": 0},
                  "length": 3,
                  "shout": "why are we shouting??",
                  "squad": ""
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let body = response.body_string().unwrap();
    let _move: responses::Move = serde_json::from_str(&body).unwrap();
}

#[test]
fn end() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client.post("/end").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn path_finding() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/move")
        .header(ContentType::JSON)
        .body(
            r#"{
                "game": {
                  "id": "game-00fe20da-94ad-11ea-bb37",
                  "ruleset": {
                    "name": "standard",
                    "version": "v.1.2.3"
                  },
                  "timeout": 500
                },
                "turn": 14,
                "board": {
                  "height": 11,
                  "width": 11,
                  "food": [
                    {"x": 5, "y": 5}, 
                    {"x": 9, "y": 0}, 
                    {"x": 2, "y": 6}
                  ],
                  "hazards": [
                    {"x": 3, "y": 2}
                  ],
                  "snakes": [
                    {
                      "id": "snake-508e96ac-94ad-11ea-bb37",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 2, "y": 0}, 
                        {"x": 2, "y": 1}, 
                        {"x": 1, "y": 1}
                      ],
                      "latency": "111",
                      "head": {"x": 2, "y": 0},
                      "length": 3,
                      "shout": "why are we shouting??",
                      "squad": ""
                    }, 
                    {
                      "id": "snake-b67f4906-94ae-11ea-bb37",
                      "name": "Another Snake",
                      "health": 16,
                      "body": [
                        {"x": 3, "y": 2}, 
                        {"x": 3, "y": 1}, 
                        {"x": 4, "y": 1}, 
                        {"x": 4, "y": 0}, 
                        {"x": 5, "y": 0}
                      ],
                      "latency": "222",
                      "head": {"x": 3, "y": 2},
                      "length": 5,
                      "shout": "I'm not really sure...",
                      "squad": ""
                    }
                  ]
                },
                "you": {
                      "id": "snake-508e96ac-94ad-11ea-bb37",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 2, "y": 0}, 
                        {"x": 2, "y": 1}, 
                        {"x": 1, "y": 1}
                      ],
                      "latency": "111",
                      "head": {"x": 2, "y": 0},
                      "length": 3,
                      "shout": "why are we shouting??",
                      "squad": ""
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let body = response.body_string().unwrap();
    let choosen_move: responses::Move = serde_json::from_str(&body).unwrap();
    let correct_move = responses::Move::new(responses::Movement::Left);
    assert_eq!(correct_move, choosen_move);
}

#[test]
fn path_finding_hard() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/move")
        .header(ContentType::JSON)
        .body(
            r#"{
                "game": {
                  "id": "game-00fe20da-94ad-11ea-bb37",
                  "ruleset": {
                    "name": "standard",
                    "version": "v.1.2.3"
                  },
                  "timeout": 500
                },
                "turn": 14,
                "board": {
                  "height": 11,
                  "width": 11,
                  "food": [
                    
                  ],
                  "hazards": [
                    {"x": 3, "y": 2}
                  ],
                  "snakes": [
                    {
                      "id": "snake-508e96ac-94ad-11ea-bb37",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 10, "y": 3},
                        {"x": 9, "y": 3}, 
                        {"x": 9, "y": 2}, 
                        {"x": 9, "y": 1}, 
                        {"x": 8, "y": 1}, 
                        {"x": 8, "y": 0}, 
                        {"x": 7, "y": 0}, 
                        {"x": 6, "y": 0}, 
                        {"x": 5, "y": 0}, 
                        {"x": 4, "y": 0}, 
                        {"x": 3, "y": 0}, 
                        {"x": 3, "y": 1}, 
                        {"x": 3, "y": 2}, 
                        {"x": 4, "y": 2}, 
                        {"x": 5, "y": 2}, 
                        {"x": 5, "y": 3}, 
                        {"x": 6, "y": 3}, 
                        {"x": 6, "y": 2}, 
                        {"x": 7, "y": 2}, 
                        {"x": 8, "y": 2}, 
                        {"x": 8, "y": 3}, 
                        {"x": 7, "y": 3}, 
                        {"x": 7, "y": 4}, 
                        {"x": 8, "y": 4}, 
                        {"x": 9, "y": 4}, 
                        {"x": 9, "y": 5}, 
                        {"x": 8, "y": 5}, 
                        {"x": 8, "y": 6}, 
                        {"x": 9, "y": 6}, 
                        {"x": 9, "y": 7}, 
                        {"x": 8, "y": 7}, 
                        {"x": 8, "y": 8}, 
                        {"x": 9, "y": 8}, 
                        {"x": 10, "y": 8} 
                      ],
                      "latency": "111",
                      "head": {"x": 10, "y": 3},
                      "length": 34,
                      "shout": "why are we shouting??",
                      "squad": ""
                    }
                  ]
                },
                "you": {
                      "id": "snake-508e96ac-94ad-11ea-bb37",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 10, "y": 3},
                        {"x": 9, "y": 3}, 
                        {"x": 9, "y": 2}, 
                        {"x": 9, "y": 1}, 
                        {"x": 8, "y": 1}, 
                        {"x": 8, "y": 0}, 
                        {"x": 7, "y": 0}, 
                        {"x": 6, "y": 0}, 
                        {"x": 5, "y": 0}, 
                        {"x": 4, "y": 0}, 
                        {"x": 3, "y": 0}, 
                        {"x": 3, "y": 1}, 
                        {"x": 3, "y": 2}, 
                        {"x": 4, "y": 2}, 
                        {"x": 5, "y": 2}, 
                        {"x": 5, "y": 3}, 
                        {"x": 6, "y": 3}, 
                        {"x": 6, "y": 2}, 
                        {"x": 7, "y": 2}, 
                        {"x": 8, "y": 2}, 
                        {"x": 8, "y": 3}, 
                        {"x": 7, "y": 3}, 
                        {"x": 7, "y": 4}, 
                        {"x": 8, "y": 4}, 
                        {"x": 9, "y": 4}, 
                        {"x": 9, "y": 5}, 
                        {"x": 8, "y": 5}, 
                        {"x": 8, "y": 6}, 
                        {"x": 9, "y": 6}, 
                        {"x": 9, "y": 7}, 
                        {"x": 8, "y": 7}, 
                        {"x": 8, "y": 8}, 
                        {"x": 9, "y": 8}, 
                        {"x": 10, "y": 8} 
                      ],
                      "latency": "111",
                      "head": {"x": 10, "y": 3},
                      "length": 34,
                      "shout": "why are we shouting??",
                      "squad": ""
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let body = response.body_string().unwrap();
    let choosen_move: responses::Move = serde_json::from_str(&body).unwrap();
    let correct_move = responses::Move::new(responses::Movement::Up);
    assert_eq!(correct_move, choosen_move);
}
