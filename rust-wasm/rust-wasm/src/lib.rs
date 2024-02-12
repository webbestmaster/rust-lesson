use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[wasm_bindgen]
struct MovePath {
    point_list: Vec<Point>,
    rest_move: i32,
}

type Landscape = Vec<Vec<i32>>;

fn get_point_value_on_landscape(point: &Point, landscape: &Landscape) -> Option<i32> {
    let Point { x, y } = point;
    let x_usize = usize::try_from(x.to_owned()).ok()?;
    let y_usize = usize::try_from(y.to_owned()).ok()?;
    let row_by_y = landscape.get(y_usize)?;
    let point_by_x = row_by_y.get(x_usize)?;

    Some(point_by_x.to_owned())
}

fn get_close_point(center: &Point, landscape: &Landscape) -> Vec<Point> {
    let Point { x, y } = center.to_owned();
    let point_up: Point = Point { x, y: y - 1 };
    let point_down: Point = Point { x, y: y + 1 };
    let point_left: Point = Point { x: x - 1, y };
    let point_right: Point = Point { x: x + 1, y };

    [point_up, point_down, point_left, point_right]
        .into_iter()
        .filter(|point_in_list| -> bool {
            get_point_value_on_landscape(point_in_list, landscape).is_some()
        })
        .collect::<Vec<Point>>()
}

fn get_available_close_point(start: &Point, move_size: i32, landscape: &Landscape) -> Vec<Point> {
    get_close_point(start, landscape)
        .into_iter()
        .filter(
            |point| match get_point_value_on_landscape(point, landscape) {
                Some(value) => move_size >= value,
                None => false,
            },
        )
        .collect::<Vec<Point>>()
}

static DEFAULT_IMPOSSIBLE_POINT: Point = Point { x: -100, y: -100 };

fn make_new_path_list(move_path: &MovePath, landscape: &Landscape) -> Vec<MovePath> {
    let last_point_in_list: &Point = move_path
        .point_list
        .last()
        .unwrap_or(&DEFAULT_IMPOSSIBLE_POINT);

    if last_point_in_list == &DEFAULT_IMPOSSIBLE_POINT {
        return Vec::new();
    }

    let close_point_list: Vec<Point> =
        get_available_close_point(last_point_in_list, move_path.rest_move, landscape);

    close_point_list
        .into_iter()
        .map(|point| -> MovePath {
            let mut new_point_list = move_path.point_list.to_owned();
            new_point_list.push(point.to_owned());

            MovePath {
                point_list: new_point_list,
                rest_move: move_path.rest_move
                    - get_point_value_on_landscape(&point, landscape).unwrap_or(0),
            }
        })
        .collect::<Vec<MovePath>>()
}

fn get_available_move_path_list_recursive(
    path_list_to_expand: Vec<MovePath>,
    mut stored_path_list: Vec<MovePath>,
    landscape: &Landscape,
) -> Vec<MovePath> {
    let mut updated_path_list_to_expand: Vec<MovePath> = Vec::new();

    path_list_to_expand
        .iter()
        .for_each(|path_to_expand_in_list: &MovePath| {
            stored_path_list.push(path_to_expand_in_list.to_owned());
        });

    path_list_to_expand
        .iter()
        .for_each(|path_to_expand_in_list: &MovePath| {
            let updated_move_path_list: Vec<MovePath> =
                make_new_path_list(path_to_expand_in_list, landscape)
                    .into_iter()
                    .filter(|updated_move_path| {
                        let is_path_already_exists =
                            stored_path_list.iter().any(|stored_move_path| -> bool {
                                match (
                                    stored_move_path.point_list.first(),
                                    stored_move_path.point_list.last(),
                                    updated_move_path.point_list.first(),
                                    updated_move_path.point_list.last(),
                                ) {
                                    (
                                        Some(stored_movie_path_start),
                                        Some(stored_movie_path_end),
                                        Some(updated_movie_path_start),
                                        Some(updated_movie_path_end),
                                    ) => {
                                        stored_movie_path_start == updated_movie_path_start
                                            && stored_movie_path_end == updated_movie_path_end
                                    }
                                    _ => false,
                                }
                            });

                        !is_path_already_exists
                    })
                    .collect::<Vec<MovePath>>();

            updated_move_path_list
                .iter()
                .for_each(|path_to_expand_in_list: &MovePath| {
                    updated_path_list_to_expand.push(path_to_expand_in_list.to_owned());
                });
        });

    if updated_path_list_to_expand.is_empty() {
        return stored_path_list;
    }

    get_available_move_path_list_recursive(updated_path_list_to_expand, stored_path_list, landscape)
}

fn vec_to_landscape(landscape_in_one_line: &Vec<i32>, landscape_width: i32) -> Landscape {
    let landscape_width_usize = usize::try_from(landscape_width).unwrap_or(landscape_in_one_line.len());

    landscape_in_one_line
        // .split(",")
        // .map(|map_shunk| {
        //     map_shunk.trim().parse::<i32>().unwrap()
        // })
        // .collect::<Vec<i32>>()
        .chunks(landscape_width_usize)
        .map(|chunk| chunk.to_owned())
        .collect::<Vec<Vec<i32>>>()
}

#[wasm_bindgen]
pub fn get_available_move_path_list(
    start_x: i32,
    start_y: i32,
    move_size: i32,
    landscape_in_one_line: Vec<i32>,
    landscape_width: i32,
) -> JsValue {
    let stored_path_list: Vec<MovePath> = Vec::new();

    let landscape: Landscape = vec_to_landscape(&landscape_in_one_line, landscape_width);
    let start: Point = Point {
        x: start_x,
        y: start_y
    };

    let move_path_list = get_available_move_path_list_recursive(
        Vec::from([MovePath {
            point_list: Vec::from([start]),
            rest_move: move_size,
        }]),
        stored_path_list,
        &landscape,
    );

    serde_wasm_bindgen::to_value(&move_path_list).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let start = Point { x: 2, y: 2 };

        let move_size: i32 = 3;
        let landscape: String = String::from("1, 2, 3, 2, 1, 1, 1, 3, 2, 1, 1, 2, 3, 2, 1, 1, 2, 3, 2, 1, 1, 2, 3, 2, 1");

        let result = get_available_move_path_list(start.x, start.y, move_size, &landscape, 5);

        print!("{:?}", result);
    }
}
