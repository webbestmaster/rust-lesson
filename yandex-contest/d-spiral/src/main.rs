use std::io;

fn main() {
    let mut list_length_str = String::new();

    io::stdin().read_line(&mut list_length_str).expect("Wrong file, should contain several line with numbers");

    let list_length = list_length_str.trim().parse::<usize>().expect("Wrong file, should contain several line with numbers");

    println!("list_length {}", list_length);

    let mut matrix = Vec::new();

    loop {
        let mut matrix_line_str = String::new();

        match io::stdin().read_line(&mut matrix_line_str) {
            Ok(_) => {
                if matrix_line_str.trim().len() == 0 {
                    break;
                }
            }
            Err(_) => break
        }

        let matrix_str_list: Vec<usize> = matrix_line_str.trim()
            .split(" ")
            .map(|x| {
                x.parse::<usize>().expect("Should be number")
            })
            .collect();

        matrix.push(matrix_str_list);
    }

    let mut direction = 'u'; // u, r, d, l
    let mut path_size = 1;
    let mut rest_path_size = 1;

    let mut pos_x: usize = (list_length - 1) / 2;
    let mut pos_y: usize = pos_x;

    println!("current_number s {:?}", &(&matrix[pos_y])[pos_x]);

    loop {
        loop {
            if direction == 'u' {
                if pos_y == 0 {
                    break;
                }

                pos_y -= 1;

                println!("current_number u {:?}", &(&matrix[pos_y])[pos_x]);
            }

            if direction == 'r' {
                pos_x += 1;

                println!("current_number r {:?}", &(&matrix[pos_y])[pos_x]);
            }

            if direction == 'd' {
                pos_y += 1;

                println!("current_number d {:?}", &(&matrix[pos_y])[pos_x]);
            }

            if direction == 'l' {
                pos_x -= 1;

                println!("current_number l {:?}", &(&matrix[pos_y])[pos_x]);
            }

            rest_path_size -= 1;

            if rest_path_size == 0 {
                break;
            }
        }

        if direction == 'u' {
            direction = 'r';
            rest_path_size = path_size;
        } else if direction == 'r' {
            direction = 'd';
            path_size += 1;
            rest_path_size = path_size;
        } else if direction == 'd' {
            direction = 'l';
            rest_path_size = path_size;
        } else if direction == 'l' {
            direction = 'u';
            path_size += 1;
            rest_path_size = path_size;
        }

        if pos_y == 0 && pos_x == 0 {
            break;
        }
    }

    println!("matrix {:?}", matrix);
}
