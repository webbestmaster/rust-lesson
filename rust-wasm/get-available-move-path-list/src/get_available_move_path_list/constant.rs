use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovePath {
    #[serde(rename = "pointList")]
    pub point_list: Vec<Point>,
    #[serde(rename = "restMove")]
    pub rest_move: i32,
}

pub mod constant_inner {
    use super::*;

    pub type Landscape = Vec<Vec<i32>>;
    
    pub static DEFAULT_IMPOSSIBLE_POINT: Point = Point { x: -100, y: -100 };
}

