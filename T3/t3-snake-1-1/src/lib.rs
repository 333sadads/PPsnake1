use wasm_bindgen::prelude::*;
use std::cmp::min;

const inf: i32 = 10000000;

const lambda_food: f64 = 1.0;
const lambda_snake: f64 = -0.25;
const lambda_border: f64 = -1.0;

const puh_gray: f64 = -10000.0;
const puh_black: f64 = -1000000.0;

#[wasm_bindgen]
pub fn greedy_snake_step(n: i32, snake: &[i32], snake_num: i32, snake_ot: &[i32], food_num: i32, foods: &[i32], round:i32) -> i32 {
    // init
    let mut snake_bank = [(0, 0); 20]; 
    let mut food_bank = [(0, 0); 50];

    for i in 0..snake_num as usize {
        snake_bank[4*i].0 = snake_ot[8*i];
        snake_bank[4*i].1 = snake_ot[8*i+1];

        snake_bank[4*i+1].0 = snake_ot[8*i+2];
        snake_bank[4*i+1].1 = snake_ot[8*i+3];

        snake_bank[4*i+2].0 = snake_ot[8*i+4];
        snake_bank[4*i+2].1 = snake_ot[8*i+5];

        snake_bank[4*i+3].0 = snake_ot[8*i+6];
        snake_bank[4*i+3].1 = snake_ot[8*i+7];

    }

    for i in 0..food_num as usize {
        food_bank[i].0 = foods[2*i];
        food_bank[i].1 = foods[2*i+1];
    }

    // cal mark
    let snake_head: (i32, i32) = (snake[0], snake[1]);
    let snake_body: (i32, i32) = (snake[2], snake[3]);
    let mut max_mark: f64 = f64::from(-inf);
    let mut ans: i32 = -1;

    let mut dir:i32 = 0;
    for dir in 0..4 {
        let mut x = snake_head.0;
        let mut y = snake_head.1;

        // 0 -> up
        if dir == 0 {
            y = y + 1;
        }
        // 1 -> left
        if dir == 1 {
            x = x - 1;
        }
        // 2 -> down
        if dir == 2 {
            y = y - 1;
        }
        // 3 -> right
        if dir == 3 {
            x = x + 1;
        }

        if (x == snake_body.0) && (y == snake_body.1) {
            continue;
        } 

        let mut mark: f64 = 0.0;
        for i in 0..(4*snake_num) as usize {
            mark  = mark + get_mark(lambda_snake, (x, y), snake_bank[i], round);
        }
        for i in 0..food_num as usize {
            mark = mark + get_mark(lambda_food, (x, y), food_bank[i], round);
        }
        mark = mark + get_mark_from_dist(lambda_border, get_dist2border((x,y), n));

        //check gray
        for i in 0..snake_num as usize {
            let snake_x = snake_bank[4*i].0;
            let snake_y = snake_bank[4*i].1;

            if get_dist((x, y), (snake_x, snake_y)) <= 1 {
                mark = mark + puh_gray;
                break;
            }
        }

        //check black
        if get_dist2border((x, y), n) == 0 {
            mark = mark + puh_black;
        } else {
            for i in 0..(4*snake_num) as usize{
                if get_dist((x, y), snake_bank[i]) == 0 {
                    mark = mark + puh_black;
                    break;
                }
            }
        }

        if mark > max_mark {
            max_mark = mark;
            ans = dir;
        }
    }
    ans
}

fn get_snake_dir(snake: &[i32]) -> i32 {
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


fn get_dist2border(pt: (i32, i32), n: i32) -> i32 {
    let mut ans = inf;
    ans = min(ans, get_dist(pt, (pt.0, 0)));
    ans = min(ans, get_dist(pt, (pt.0, n+1)));
    ans = min(ans, get_dist(pt, (0, pt.1)));
    ans = min(ans, get_dist(pt, (n+1, pt.1)));
    ans
}

fn get_dist(pt1: (i32, i32), pt2: (i32, i32)) -> i32 {
    (pt1.0 - pt2.0).abs() + (pt1.1 - pt2.1).abs()
}

fn get_mark(lambda: f64, pt1: (i32, i32), pt2: (i32, i32), round: i32) -> f64 {
    let dist = get_dist(pt1, pt2);
    if round > 10 {
        100.0 * lambda / (f64::from(dist) * f64::from(dist) * f64::from(dist) + 0.1)        
    } else {
        100.0 * lambda / (f64::from(dist) * f64::from(dist) * f64::from(dist) * f64::from(dist)  + 0.1)         
    }
}

fn get_mark_from_dist(lambda: f64, dist: i32) -> f64 {
    100.0 * lambda / (f64::from(dist) * f64::from(dist) + 0.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
