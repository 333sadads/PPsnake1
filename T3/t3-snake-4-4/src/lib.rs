use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_step(n: i32, snake: &[i32], snake_num: i32, 
    snake_ot: &[i32], food_num: i32, foods: &[i32], round:i32) -> i32 {
    let pos = get_pos(snake);
    if (pos != 2) && (foods[1] > snake[1]) {
        0
    } else if (pos != 3) && (foods[0] < snake[0]) {
        1
    } else if (pos != 0) && (foods[1] < snake[1]) {
        2
    } else if (pos != 1) && (foods[0] > snake[0]){
        3
    } else {
        (pos + 1)%4
    }
}

fn get_pos(snake: &[i32]) -> i32 {
    if snake[0] == snake[2] {
        if snake[1] > snake[3] {
            0 
        } else {
            2
        }
    } else {
        if snake[0] > snake[2] {
            3
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
