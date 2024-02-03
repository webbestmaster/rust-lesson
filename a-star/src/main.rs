// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]

#[derive(Debug, PartialEq, Clone)]
struct Point(i32, i32);

type Path = Vec<Point>;
type PathList = Vec<Path>;
type Map = Vec<Vec<char>>;

fn get_char_by_point(map: &Map, point: &Point) -> Option<char> {
    let &Point(x, y) = point;

    let row_by_y: &Vec<char> = map.get(y as usize)?;
    let char_by_x: &char = row_by_y.get(x as usize)?;

    Some(*char_by_x)
}

fn get_is_forbidden_symbol(symbol: char) -> bool {
    symbol == '#'
}

fn get_is_point_in_path_list(point: &Point, path_list: &PathList) -> bool {
    return path_list
        .iter()
        .any(|path| -> bool { path.contains(point) });
}

fn get_next_points(map: &Map, path_list: &PathList, point: &Point) -> Vec<Point> {
    let &Point(x, y) = point;

    Vec::from([
        Point(x - 1, y),
        Point(x + 1, y),
        Point(x, y - 1),
        Point(x, y + 1),
    ])
    .into_iter()
    .filter(|point_in_filter: &Point| -> bool {
        match get_char_by_point(map, point_in_filter) {
            Some(char_in_map) => {
                if get_is_forbidden_symbol(char_in_map) {
                    return false;
                }

                if get_is_point_in_path_list(point_in_filter, path_list) {
                    return false;
                }

                true
            }
            None => false,
        }
    })
    .collect::<Vec<Point>>()
}

fn main() {
    let map: Map = Vec::from([
        Vec::from(['.', '#', '.', '.', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '#', '.', '#', '.']),
        Vec::from(['.', '.', '.', '#', '.']),
        Vec::from(['.', '.', '.', '#', '.']),
    ]);

    let map2: Map = "1 2\n3 4"
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|chunk| chunk.chars().collect::<Vec<char>>())
        .collect::<Map>();

    println!("map2 {:?}", map2);

    // let point_begin = Point(2, 2);

    let point_end = Point(4, 4);

    // let mut new_points: Vec<Point> = Vec::new();

    let mut path_list: PathList = Vec::new();

    let first_path = vec![Point(0, 0)];
    path_list.push(first_path);

    let mut new_path_list: PathList = Vec::new();

    loop {
        new_path_list.clear();

        // get all points from all pathes
        path_list.iter().for_each(|path: &Vec<Point>| {
            let last_point = match path.last() {
                Some(point) => point,
                None => return,
            };

            let new_points = get_next_points(&map, &path_list, last_point);

            new_points.iter().for_each(|new_point| {
                let mut cloned_path: Vec<Point> = path.clone();
                cloned_path.push(new_point.to_owned());
                new_path_list.push(cloned_path);
            });
        });

        // check is reached
        let reached_path_list: PathList = new_path_list
            .iter()
            .filter(|path| -> bool {
                let last_point = match path.last() {
                    Some(point) => point,
                    None => return false,
                };

                *last_point == point_end
            })
            .map(|data| -> Vec<Point> { data.to_owned() })
            .collect::<PathList>();

        if !reached_path_list.is_empty() {
            println!("reached_path_list {:?}", reached_path_list);
            break;
        }

        if new_path_list.is_empty() {
            println!("can not get new_path_list {:?}", new_path_list);
            break;
        }

        path_list.clear();

        // println!("new_path_list {}", new_path_list.len());

        new_path_list.iter().for_each(|path| {
            path_list.push(path.to_owned());
        });
    }
}
