use wasm_bindgen::prelude::*;
// #[derive(Debug, PartialEq)]
// pub enum Direction {
//     Up = 0,
//     Right = 3,
//     Down = 2,
//     Left = 1,
// }
#[wasm_bindgen]
pub fn greedy_snake_move(snake: &[i32], food: &[i32]) -> i32 {
    let head_x = snake[0];
    let head_y = snake[1];
    let food_x = food[0];
    let food_y = food[1];

    // Calculate the relative position of the food to the snake's head
    let delta_x = food_x - head_x;
    let delta_y = food_y - head_y;

    // Prioritize the direction with the larger distance
    if delta_x.abs() > delta_y.abs() {
        // Horizontal direction first
        let direction = if delta_x > 0 { 3 } else { 1 };
        if is_valid_move(&snake, direction) {
            return direction;
        }
        // If preferred direction is invalid, try vertical direction
        let fallback_direction = if delta_y > 0 { 0 } else { 2 };
        if is_valid_move(&snake, fallback_direction) {
            return fallback_direction;
        }
    } else {
        // Vertical direction first
        let direction = if delta_y > 0 { 0 } else { 2 };
        if is_valid_move(&snake, direction) {
            return direction;
        }
        // If preferred direction is invalid, try horizontal direction
        let fallback_direction = if delta_x > 0 { 3 } else { 1 };
        if is_valid_move(&snake, fallback_direction) {
            return fallback_direction;
        }
    }

    // If all preferred directions are invalid, choose the first available safe direction
    for dir in 0..4 {
        if is_valid_move(&snake, dir) {
            return dir;
        }
    }

    // If no move is possible, default to moving up (this should not occur)
    return 0;
}

#[wasm_bindgen]
pub fn is_valid_move(snake: &[i32], direction: i32) -> bool {
    let head_x = snake[0];
    let head_y = snake[1];

    // Calculate new snake head position
    let mut new_x = head_x;
    let mut new_y = head_y;

    match direction {
        2 => new_y -= 1, // Down
        0 => new_y += 1, // Up
        1 => new_x -= 1, // Left
        3 => new_x += 1, // Right
        _ => {}
    }

    // Check if new position is out of bounds (assuming an 8x8 grid, coordinates 1-8)
    if new_x < 1 || new_x > 8 || new_y < 1 || new_y > 8 {
        return false;
    }

    // Check if the new head position collides with the snake's body (ignoring the current head)
    if new_x == snake[2] && new_y == snake[3] {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_right() {
        let snake = [1, 1, 1, 2, 1, 3, 1, 4]; // 蛇头在(1,1)，脖子在(1,0)
        let food = [1, 5];        // 食物在右侧
        assert_eq!(greedy_snake_move(&snake, &food), 3);
    }

    
}