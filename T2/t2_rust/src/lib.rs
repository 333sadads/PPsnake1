use wasm_bindgen::prelude::*;
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;
#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake: &[i32], fruit: &[i32], barriers: &[i32]) -> i32 {
    let path = find_path_with_a_star(snake, fruit, barriers);
    if path.is_none() || path.as_ref().unwrap().len() <= 1 {
        return -1; // 不可达
    }

    let next_step = path.unwrap()[1];
    let head_x = snake[0];
    let head_y = snake[1];

    for dir in 0..4 {
        if head_x + DX[dir] == next_step[0] && head_y + DY[dir] == next_step[1] {
            return dir as i32;
        }
    }

    -1 // 理论上不应该执行到这里
}

const DX: [i32; 4] = [0, -1, 0, 1];
const DY: [i32; 4] = [1, 0, -1, 0];

fn find_path_with_a_star(snake: &[i32], fruit: &[i32], barriers: &[i32]) -> Option<Vec<[i32; 2]>> {
    let head_x = snake[0];
    let head_y = snake[1];
    let fruit_x = fruit[0];
    let fruit_y = fruit[1];

    if head_x == fruit_x && head_y == fruit_y {
        return Some(vec![[head_x, head_y]]);
    }

    let mut open_list = BinaryHeap::new();
    let mut all_nodes = HashMap::new();

    let mut snake_body = HashSet::new();
    snake_body.insert(format!("{},{}", snake[2], snake[3]));

    let mut barrier_set = HashSet::new();
    for i in (0..barriers.len()).step_by(2) {
        barrier_set.insert(format!("{},{}", barriers[i], barriers[i + 1]));
    }

    let start_node = Node::new(head_x, head_y, 0, manhattan_distance(head_x, head_y, fruit_x, fruit_y));
    open_list.push(start_node.clone());
    all_nodes.insert(format!("{},{}", head_x, head_y), start_node);

    while let Some(current_node) = open_list.pop() {
        if current_node.x == fruit_x && current_node.y == fruit_y {
            return Some(reconstruct_path(&current_node));
        }

        for dir in 0..4 {
            let new_x = current_node.x + DX[dir];
            let new_y = current_node.y + DY[dir];

            if new_x <= 0 || new_x >= 9 || new_y <= 0 || new_y >= 9 ||
                snake_body.contains(&format!("{},{}", new_x, new_y)) ||
                barrier_set.contains(&format!("{},{}", new_x, new_y)) {
                continue;
            }

            let key = format!("{},{}", new_x, new_y);
            let neighbor = all_nodes.entry(key.clone()).or_insert(Node::new(new_x, new_y, i32::MAX, 0));

            let tentative_g = current_node.g + 1;
            if tentative_g < neighbor.g {
                neighbor.parent = Some(Box::new(current_node.clone()));
                neighbor.g = tentative_g;
                neighbor.h = manhattan_distance(new_x, new_y, fruit_x, fruit_y);
                neighbor.f = neighbor.g + neighbor.h;

                if !open_list.iter().any(|n| n.x == neighbor.x && n.y == neighbor.y) {
                    open_list.push(neighbor.clone());
                }
            }
        }
    }

    None // 没有找到路径
}

fn reconstruct_path(end_node: &Node) -> Vec<[i32; 2]> {
    let mut path = Vec::new();
    let mut current = Some(end_node.clone());
    while let Some(node) = current {
        path.insert(0, [node.x, node.y]);
        current = node.parent.map(|p| *p);
    }
    path
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[derive(Clone)]
struct Node {
    x: i32,
    y: i32,
    g: i32,
    h: i32,
    f: i32,
    parent: Option<Box<Node>>,
}

impl Node {
    fn new(x: i32, y: i32, g: i32, h: i32) -> Self {
        Node { x, y, g, h, f: g + h, parent: None }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_right() {
        let snake = [1, 4, 1, 3, 1, 2, 1, 1]; // 蛇头在(1,1)，脖子在(1,0)
        let food = [5, 5];        // 食物在右侧
        let b=[2, 7, 2, 6, 3, 7, 3, 6, 4, 6, 5, 6, 6, 6, 7, 6, 4, 5, 4, 4, 4, 3, 5, 4];
        assert_eq!(greedy_snake_move_barriers(&snake,&food,&b)>0, true);
    }

    
}