use crate::config::Configuration;
use crate::pathing::action::{moveset_2d_cardinal, MoveAction};
use crate::pathing::algorithm::PathCalculator;
use crate::pathing::math::Vector2i;
use crate::pathing::world::FlatSpace;
use crate::vec2i;
use std::rc::Rc;

#[test]
fn scenario_from_str() {
    let scenario = PathfindingScenario2D::new(vec![
        "_____",
        "_____",
        "_____",
        "_____",
        "_____"
    ], moveset_2d_cardinal());

    let conv_path1 = scenario.to_positions(vec2i!(0, 0), vec2i!(4, 0), vec![
        "OOOOO",
        "_____",
        "_____",
        "_____",
        "_____"
    ]);
    assert_eq!(conv_path1, vec![vec2i!(0, 0), vec2i!(1 ,0), vec2i!(2, 0), vec2i!(3, 0), vec2i!(4, 0)],
               "Failed to convert FlatSpace to a path");
    let conv_path2 = scenario.to_positions(vec2i!(0, 0), vec2i!(0, 4), vec![
        "O____",
        "O____",
        "O____",
        "O____",
        "O____"
    ]);
    assert_eq!(conv_path2, vec![vec2i!(0, 0), vec2i!(0, 1), vec2i!(0, 2), vec2i!(0, 3), vec2i!(0, 4)],
               "Failed to convert FlatSpace to a path");

    let conv_path3 = scenario.to_positions(vec2i!(0, 0), vec2i!(4, 4), vec![
        "O_OOO",
        "O_O_O",
        "O_O_O",
        "O_O_O",
        "OOO_O"
    ]);
    assert_eq!(conv_path3, vec![
        vec2i!(0, 0), vec2i!(0, 1), vec2i!(0, 2), vec2i!(0, 3), vec2i!(0, 4),
        vec2i!(1, 4),
        vec2i!(2, 4), vec2i!(2, 3), vec2i!(2, 2), vec2i!(2, 1), vec2i!(2, 0),
        vec2i!(3, 0),
        vec2i!(4, 0), vec2i!(4, 1), vec2i!(4, 2), vec2i!(4, 3), vec2i!(4, 4)
    ], "Failed to convert FlatSpace to a path");
}

#[test]
fn pathfinder_trivial() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "O_G"
    ], moveset_2d_cardinal());

    scenario.eval_success(vec2i!(0, 0), vec2i!(2, 0));
}

#[test]
fn pathfinder_trivial_large() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "O____G",
        "_____",
        "_____",
        "_____",
        "O___O"
    ], moveset_2d_cardinal());

    scenario.eval_success(vec2i!(0, 0), vec2i!(4, 4));
    scenario.eval_success(vec2i!(0, 4), vec2i!(4, 4));
    scenario.eval_success(vec2i!(4, 0), vec2i!(4, 4));
}

#[test]
fn pathfinder_trivial_blocked() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "OX_XG",
        "XX_XX",
        "_____",
        "XX_XX",
        "OX_XO"
    ], moveset_2d_cardinal());

    scenario.eval_failure(vec2i!(0,0), vec2i!(4, 4));
    scenario.eval_failure(vec2i!(0,4), vec2i!(4, 4));
    scenario.eval_failure(vec2i!(4,0), vec2i!(4, 4));
    scenario.eval_failure(vec2i!(2,2), vec2i!(4, 4));
}

#[test]
fn pathfinder_maze_simple() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "OX_XG",
        "_X_X_",
        "_X_X_",
        "_XXX_",
        "_____"
    ], moveset_2d_cardinal());

    scenario.eval_success(vec2i!(0, 0), vec2i!(4, 4));
    scenario.eval_success(vec2i!(0, 4), vec2i!(4, 4));
}

#[test]
fn pathfinder_maze_shortcut() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "OX_XG",
        "_X_X_",
        "_____",
        "_XXX_",
        "_____"
    ], moveset_2d_cardinal());

    scenario.eval(vec2i!(0, 0), vec2i!(4, 0), vec![
        "O___O",
        "O___O",
        "OOOOO",
        "_____",
        "_____"
    ]);
}

#[test]
fn pathfinder_maze_complex() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "OX____XXXX",
        "_X__X__XX_",
        "_X__X_____",
        "_XX_XXX__X",
        "____XG__XX",
        "_X_XXXX__X",
        "XX____X___",
        "____X___XX",
        "__X_XXX___",
        "XXX_____XX"
    ], moveset_2d_cardinal());

    scenario.eval(vec2i!(0, 0), vec2i!(4, 5), vec![
        "O__OOO____",
        "O__O_O____",
        "O__O_OOO__",
        "O__O___O__",
        "OOOO_OOO__",
        "__________",
        "__________",
        "__________",
        "__________",
        "__________"
    ]);
}

struct PathfindingScenario2D {
    calc: PathCalculator<Vector2i, FlatSpace>,
    moveset: Rc<Vec<MoveAction<Vector2i>>>,
    environment: Vec<&'static str>
}

impl PathfindingScenario2D {
    fn new(environment: Vec<&'static str>, moveset: Vec<MoveAction<Vector2i>>) -> PathfindingScenario2D {
        let config = Rc::new(Configuration::new());
        let space = Rc::new(FlatSpace::new(environment.clone(), config.clone()));
        let moves = Rc::new(moveset);
        PathfindingScenario2D {
            calc: PathCalculator::new(moves.clone(), config, space),
            moveset: moves,
            environment
        }
    }

    fn eval(&mut self, start: Vector2i, end: Vector2i, follow: Vec<&'static str>) {
        // calculate
        let out = self.calc.calculate(start, end);
        assert!(out.is_some(), "Pathfinder failed to calculate a path entirely");
        let path = out.unwrap();
        assert!(path.len() > 0, "Pathfinder returned an empty path");

        // now compare paths
        let target_path: Vec<Vector2i> = self.to_positions(start, end, follow);
        let res_path: Vec<Vector2i> = path.iter().map(|pn| pn.pos).collect();
        assert_eq!(target_path.len(), res_path.len(),
                   "Paths did not have equal length:\n---Expected:\n{}---Actual:\n{}",
                   self.draw_path(&target_path),
                   self.draw_path(&res_path)
        );
        // compare the individual positions
        for i in 0..path.len() {
            let cmp = target_path.get(i);
            let res = path.get(i);
            assert!(cmp.is_some() && res.is_some(), "Unexpectedly failed to get a path, possible deviation detected");
            assert_eq!(*cmp.unwrap(),
                       res.unwrap().pos,
                       "Pathfinder did not choose the optimal path:\n---Expected:\n{}---Actual:\n{}",
                       self.draw_path(&target_path),
                       self.draw_path(&res_path)
            );
        }
        // clean up
        println!("---SUCCESS---\n{}", self.draw_path(&target_path));
        self.calc.reset()
    }

    fn to_positions(&self, start: Vector2i, end: Vector2i, follow: Vec<&'static str>) -> Vec<Vector2i> {
        let mut out_path: Vec<Vector2i> = Vec::new();
        let mut current = start;
        out_path.push(start);

        while current != end {
            let mut pushed = false;
            for m in self.moveset.iter() {
                let new_pos = current + m.offset;
                // only add if this spot has not been added before
                if !out_path.contains(&new_pos) && let Some(row) = follow.get(new_pos.y as usize) &&
                    let Some(cc) = row.chars().nth(new_pos.x as usize) && cc == 'O' {
                    out_path.push(new_pos.clone());
                    current = new_pos;
                    pushed = true;
                    break;
                }
            }
            // do not continue looping if we're stuck
            if !pushed { break }
        }

        out_path
    }

    fn draw_path(&self, path: &Vec<Vector2i>) -> String {
        let mut environ_str: Vec<String> = self.environment.iter()
            .map(|row| row.to_string()).collect();

        // iterate through the positions
        let mut last = Vector2i::zero(); // last difference
        for i in 0..path.len() {
            let pos = path[i];
            let c: char;

            // otherwise, assign the values
            if i == 0 {
                c = '@';
            } else if i == path.len()-1 {
                c = 'G';
            } else if i+1 < path.len() { // peek ahead
                if i > 0 {
                    last = pos - path[i-1];
                }
                let direction = pos - path[i+1] - last;
                c = '*'
                /*
                c = match direction.clamp_comp(-1, 1) {
                    vec2i!(0, 1) | vec2i!(0, -1) => '│',
                    vec2i!(1, 0) | vec2i!(-1, 0) => '─',
                    vec2i!(1, 1) => '└',
                    vec2i!(-1, 1) => '┘',
                    vec2i!(-1, -1) => '┐',
                    vec2i!(1, -1) => '┌',
                    _ => '?'
                }
                 */
            } else {
                c = '?';
            }

            let row = &mut environ_str[pos.y as usize];
            row.replace_range(
                row.char_indices().nth(pos.x as usize)
                    .map(|(p, ch)| p..p + ch.len_utf8())
                    .unwrap(),
                c.encode_utf8(&mut [0u8; 4])
            )
        }

        // then combine into a single string
        let mut combined = String::new();
        for s in environ_str {
            combined.push_str(&s);
            combined.push('\n');
        }

        combined
    }

    fn eval_success(&mut self, start: Vector2i, end: Vector2i) {
        let out = self.calc.calculate(start, end);
        assert!(out.is_some(), "Pathfinder failed to calculate a path entirely");
        let path = out.unwrap();
        assert!(path.len() > 0, "Pathfinder returned an empty path");
        assert_eq!(path.last().unwrap().pos, end, "Pathfinder did not reach the end successfully");

        let pos_path = path.iter().map(|pn| pn.pos).collect();
        println!("---SUCCESS---\n{}", self.draw_path(&pos_path));
        self.calc.reset()
    }

    fn eval_failure(&mut self, start: Vector2i, end: Vector2i) {
        let out = self.calc.calculate(start, end);
        assert!(out.is_none(), "Pathfinder calculated an impossible path");
        self.calc.reset();
    }
}