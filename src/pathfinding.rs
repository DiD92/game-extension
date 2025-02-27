use godot::prelude::*;
use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    vec::Vec,
};

#[derive(GodotClass)]
#[class(no_init)]
struct RustPathfinder;

#[godot_api]
impl RustPathfinder {
    #[func]
    fn compute_movement_range(
        origin: Vector2i,
        unit_move_range: i32,
        unit_move_multiplier: i32,
        unit_army_id: StringName,
        grid_data: Variant,
        unit_states: Variant,
        cost_function: Callable,
    ) -> Array<Vector2i> {
        let grid_bounds = Rect2i::from_variant(&grid_data.call("get_grid_bounds", &[]));
        let solid_nodes = Dictionary::from_variant(&grid_data.call("get_solid_nodes", &[]));

        if unit_move_range <= 0 || unit_move_multiplier <= 0 {
            return array![origin];
        }

        let mut move_queue = PriorityQueue::with_capacity((unit_move_range as usize) << 2);
        let mut move_visited = HashMap::new();

        let eff_move_range = unit_move_range * unit_move_multiplier;

        move_queue.push(origin, Reverse(0_i32));

        while let Some((node, Reverse(path_cost))) = move_queue.pop() {
            if path_cost <= eff_move_range {
                move_visited.insert(node, path_cost);

                for neighbour in Self::get_neighbours(node, grid_bounds) {
                    let args = varray![node, neighbour, unit_army_id, grid_data, unit_states];
                    let cost_to_move: i32 = cost_function.callv(&args).to();
                    let neighbour_cost = cost_to_move + path_cost;

                    if (move_visited.get(&neighbour).unwrap_or(&9999999) >= &neighbour_cost)
                        && !solid_nodes.contains_key(neighbour)
                    {
                        move_queue.push_increase(neighbour, Reverse(neighbour_cost));
                    }
                }
            }
        }

        move_visited.into_keys().collect()
    }

    #[func]
    fn compute_action_range(
        move_visited: Array<Vector2i>,
        unit_range: Vector2i,
        grid_bounds: Rect2i,
    ) -> Array<Vector2i> {
        let mut action_queue = PriorityQueue::with_capacity(move_visited.len());

        let mut frontier_visited = HashMap::new();
        let mut action_visited = HashSet::new();

        for cell in move_visited.iter_shared() {
            action_queue.push((cell, cell), Reverse(0_i32));
        }

        while let Some(((pivot, target), Reverse(path_cost))) = action_queue.pop() {
            if path_cost <= unit_range.y {
                if !action_visited.contains(&target)
                    && (path_cost >= unit_range.x && path_cost <= unit_range.y)
                {
                    action_visited.insert(target);
                }

                for neighbour in Self::get_neighbours(target, grid_bounds) {
                    let target_cost = Self::manhattan_distance(pivot, neighbour);

                    if target_cost < unit_range.x {
                        if *frontier_visited.get(&neighbour).unwrap_or(&0) < target_cost {
                            frontier_visited.insert(neighbour, target_cost);
                            action_queue.push((pivot, neighbour), Reverse(target_cost));
                        }
                    } else if !action_visited.contains(&neighbour) {
                        action_queue.push((pivot, neighbour), Reverse(target_cost));
                    }
                }
            }
        }

        action_visited.into_iter().collect()
    }

    #[func]
    fn compute_movement_path(
        from: Vector2i,
        to: Vector2i,
        unit_node: Variant,
        grid_data: Variant,
        army_manager: Variant,
        valid_cells: Dictionary,
        cost_function: Callable,
    ) -> Array<Vector2i> {
        if from == to {
            return Array::from(&[from]);
        }

        let grid_bounds = Rect2i::from_variant(&grid_data.call("get_grid_bounds", &[]));
        let solid_nodes = Dictionary::from_variant(&grid_data.call("get_solid_nodes", &[]));

        let mut path: Vec<Vector2i> = vec![from];

        let mut path_queue = PriorityQueue::with_capacity(4);
        let mut min_cost = i32::MAX;
        let mut visited_nodes = HashSet::new();

        path_queue.push(vec![from], Reverse(0_i32));

        while let Some((node_path, Reverse(path_cost))) = path_queue.pop() {
            if path_cost < min_cost {
                let node = node_path[node_path.len() - 1];

                visited_nodes.insert(node);

                if node == to {
                    min_cost = path_cost;
                    path = node_path;
                } else {
                    for neighbour in Self::get_neighbours(node, grid_bounds) {
                        if !valid_cells.contains_key(neighbour) {
                            continue;
                        }

                        if !visited_nodes.contains(&neighbour)
                            && !solid_nodes.contains_key(neighbour)
                        {
                            let args = varray![node, neighbour, unit_node, grid_data, army_manager];
                            let cost_to_move: i32 = cost_function.callv(&args).to();
                            let new_path = {
                                let mut path = node_path.clone();
                                path.push(neighbour);
                                path
                            };

                            path_queue.push(new_path, Reverse(path_cost + cost_to_move));
                        }
                    }
                }
            }
        }

        path.into_iter().collect()
    }

    #[func]
    fn manhattan_distance(from: Vector2i, to: Vector2i) -> i32 {
        (from.x - to.x).abs() + (from.y - to.y).abs()
    }
}

impl RustPathfinder {
    fn get_neighbours(cell: Vector2i, grid_bounds: Rect2i) -> impl Iterator<Item = Vector2i> {
        if grid_bounds.contains_point(cell) {
            vec![
                Vector2i::UP,
                Vector2i::DOWN,
                Vector2i::LEFT,
                Vector2i::RIGHT,
            ]
            .into_iter()
        } else {
            Vec::<Vector2i>::with_capacity(0).into_iter()
        }
        .map(move |item| cell + item)
        .filter(move |item| grid_bounds.contains_point(*item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_neighbours {
        use super::*;

        #[test]
        fn get_neighbours_works() {
            let grid_start = Vector2i::new(0, 0);
            let grid_end = Vector2i::new(5, 10);
            let grid_bounds = Rect2i::new(grid_start, grid_end);

            // Neighours should be returned in UP, DOWN, LEFT, RIGHT order.
            let point_1 = Vector2i::new(1, 1);
            let neighbours_1 =
                RustPathfinder::get_neighbours(point_1, grid_bounds).collect::<Vec<_>>();
            assert_eq!(
                neighbours_1,
                vec![
                    Vector2i::new(1, 0),
                    Vector2i::new(1, 2),
                    Vector2i::new(0, 1),
                    Vector2i::new(2, 1)
                ]
            );
        }

        #[test]
        fn get_neighbours_returns_empty_iter_when_outside_grid() {
            let grid_start = Vector2i::new(0, 0);
            let grid_end = Vector2i::new(5, 10);
            let grid_bounds = Rect2i::new(grid_start, grid_end);

            // If the cell is completely outside of the grid, no neighours are returned
            let point_2 = Vector2i::new(-2, -2);
            let neighbours_2 =
                RustPathfinder::get_neighbours(point_2, grid_bounds).collect::<Vec<_>>();
            assert_eq!(neighbours_2, vec![]);
        }

        #[test]
        fn get_neighbours_returns_only_neighbours_inside_grid() {
            let grid_start = Vector2i::new(0, 0);
            let grid_end = Vector2i::new(5, 10);
            let grid_bounds = Rect2i::new(grid_start, grid_end);

            // If the cell is at the borders of the grid, only neighbours inside are returned
            let point_3 = Vector2i::new(0, 0);
            let neighbours_3 =
                RustPathfinder::get_neighbours(point_3, grid_bounds).collect::<Vec<_>>();
            assert_eq!(neighbours_3, vec![Vector2i::new(0, 1), Vector2i::new(1, 0)]);
            let point_4 = Vector2i::new(3, 0);
            let neighbours_4 =
                RustPathfinder::get_neighbours(point_4, grid_bounds).collect::<Vec<_>>();
            assert_eq!(
                neighbours_4,
                vec![
                    Vector2i::new(3, 1),
                    Vector2i::new(2, 0),
                    Vector2i::new(4, 0)
                ]
            );
        }
    }

    mod manhattan_distance {
        use super::*;

        #[test]
        fn manhattan_distance_works() {
            let point_1 = Vector2i::new(1, 2);
            let point_2 = Vector2i::new(4, 9);
            let distance = RustPathfinder::manhattan_distance(point_1, point_2);
            assert_eq!(distance, 10);

            let point_1 = Vector2i::new(-3, -2);
            let point_2 = Vector2i::new(-5, -2);
            let distance = RustPathfinder::manhattan_distance(point_1, point_2);
            assert_eq!(distance, 2);

            let point_1 = Vector2i::new(0, 0);
            let point_2 = Vector2i::new(0, 0);
            let distance = RustPathfinder::manhattan_distance(point_1, point_2);
            assert_eq!(distance, 0);

            let point_1 = Vector2i::new(-3, 0);
            let point_2 = Vector2i::new(4, 1);
            let distance = RustPathfinder::manhattan_distance(point_1, point_2);
            assert_eq!(distance, 8);
        }
    }
}
