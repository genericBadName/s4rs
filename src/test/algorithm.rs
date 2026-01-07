use crate::config::Configuration;
use crate::pathing::action::{moveset_2d_cardinal, MoveAction, Moveset2D, Moveset, SpatialAction};
use crate::pathing::algorithm::PathCalculator;
use crate::pathing::math::Vector2i;
use crate::pathing::world::FlatSpace;
use crate::vec2i;
use crate::pathing::data::PathNode;

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
    assert_eq!(conv_path1, vec![
        PathNode::new(SpatialAction::new_root(vec2i!(0, 0))),
        mv(vec2i!(1, 0), Moveset2D::Right),
        mv(vec2i!(2, 0), Moveset2D::Right),
        mv(vec2i!(3, 0), Moveset2D::Right),
        mv(vec2i!(4, 0), Moveset2D::Right),
    ],
               "Failed to convert FlatSpace to a path"
    );
    let conv_path2 = scenario.to_positions(vec2i!(0, 0), vec2i!(0, 4), vec![
        "O____",
        "O____",
        "O____",
        "O____",
        "O____"
    ]);
    assert_eq!(conv_path2, vec![
        PathNode::new(SpatialAction::new_root(vec2i!(0, 0))),
        mv(vec2i!(0, 1), Moveset2D::Down),
        mv(vec2i!(0, 2), Moveset2D::Down),
        mv(vec2i!(0, 3), Moveset2D::Down),
        mv(vec2i!(0, 4), Moveset2D::Down)
    ], 
               "Failed to convert FlatSpace to a path"
    );

    let conv_path3 = scenario.to_positions(vec2i!(0, 0), vec2i!(4, 4), vec![
        "O_OOO",
        "O_O_O",
        "O_O_O",
        "O_O_O",
        "OOO_O"
    ]);
    assert_eq!(conv_path3, vec![
        PathNode::new(SpatialAction::new_root(vec2i!(0, 0))), 
        mv(vec2i!(0, 1), Moveset2D::Down), 
        mv(vec2i!(0, 2), Moveset2D::Down), 
        mv(vec2i!(0, 3), Moveset2D::Down), 
        mv(vec2i!(0, 4), Moveset2D::Down),

        mv(vec2i!(1, 4), Moveset2D::Right),

        mv(vec2i!(2, 4), Moveset2D::Right),
        mv(vec2i!(2, 3), Moveset2D::Up),
        mv(vec2i!(2, 2), Moveset2D::Up),
        mv(vec2i!(2, 1), Moveset2D::Up),
        mv(vec2i!(2, 0), Moveset2D::Up),

        mv(vec2i!(3, 0), Moveset2D::Right),

        mv(vec2i!(4, 0), Moveset2D::Right),
        mv(vec2i!(4, 1), Moveset2D::Down),
        mv(vec2i!(4, 2), Moveset2D::Down),
        mv(vec2i!(4, 3), Moveset2D::Down),
        mv(vec2i!(4, 4), Moveset2D::Down),
    ], "Failed to convert FlatSpace to a path");
}

fn mv(pos: Vector2i, action: Moveset2D) -> PathNode<Vector2i> {
    PathNode::new(SpatialAction::new(pos, action.of()))
}

#[test]
fn pathfinder_trivial() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "O_G"
    ], moveset_2d_cardinal());

    scenario.eval(vec2i!(0, 0), vec2i!(2, 0), vec![
        "OOO"
    ]);
}

#[test]
fn pathfinder_trivial_large() {
    let mut scenario = PathfindingScenario2D::new(vec![
        "O___G",
        "_____",
        "_____",
        "_____",
        "____O"
    ], moveset_2d_cardinal());

    scenario.eval(vec2i!(0, 0), vec2i!(4, 0), vec![
        "OOOOO",
        "     ",
        "     ",
        "     ",
        "     "
    ]);
    scenario.eval(vec2i!(4, 4), vec2i!(4, 0), vec![
        "    O",
        "    O",
        "    O",
        "    O",
        "    O"
    ]);
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

    scenario.eval(vec2i!(0, 0), vec2i!(5, 4), vec![
        "O_________",
        "O_________",
        "O_________",
        "O_________",
        "OOO__OOO__",
        "__O____O__",
        "__OOOO_O__",
        "_____OOO__",
        "__________",
        "__________"
    ]);
}

struct PathfindingScenario2D {
    calc: PathCalculator<Vector2i, FlatSpace>,
    moveset: Moveset<Vector2i>,
    environment: Vec<&'static str>
}

impl PathfindingScenario2D {
    fn new(environment: Vec<&'static str>, moveset: Moveset<Vector2i>) -> PathfindingScenario2D {
        let config = Configuration::new();
        let space = Box::new(FlatSpace::new(environment.clone(), config));
        let moves = moveset;
        PathfindingScenario2D {
            calc: PathCalculator::new(moves.clone(), config, space),
            moveset: moves,
            environment
        }
    }

    fn eval(&mut self, start: Vector2i, end: Vector2i, follow: Vec<&'static str>) {
        // calculate
        let out = self.calc.calculate(start, end);
        assert!(out.is_ok(), "Pathfinding failed with error: {}", out.unwrap_err());
        let path = out.unwrap();
        assert!(path.len() > 0, "Pathfinder returned an empty path");

        // now compare paths
        let target_path: Vec<PathNode<Vector2i>> = self.to_positions(start, end, follow);
        assert_eq!(target_path.len(), path.len(),
                   "Paths did not have equal length:\n---Expected:\n{}---Actual:\n{}",
                   self.draw_path(&target_path),
                   self.draw_path(&path)
        );
        // compare the individual positions
        for i in 0..path.len() {
            let cmp = target_path.get(i);
            let res = path.get(i);
            assert!(cmp.is_some() && res.is_some(), "Unexpectedly failed to get a path, possible deviation detected");
            assert_eq!(cmp.unwrap().action.pos,
                       res.unwrap().action.pos,
                       "Pathfinder did not choose the optimal path:\n---Expected:\n{}---Actual:\n{}",
                       self.draw_path(&target_path),
                       self.draw_path(&path)
            );
        }
        // clean up
        println!("---SUCCESS---\n{}", self.draw_path(&target_path));
        self.calc.reset()
    }

    fn to_positions(&self, start: Vector2i, end: Vector2i, follow: Vec<&'static str>) -> Vec<PathNode<Vector2i>> {
        let mut out_path: Vec<PathNode<Vector2i>> = Vec::new();
        let mut positions: Vec<Vector2i> = Vec::new();
        let mut current = start;
        out_path.push(PathNode::new(SpatialAction::new_root(start)));
        positions.push(start);

        while current != end {
            let mut pushed = false;
            for m in self.moveset.iter() {
                let new_pos = current + m.offset;
                // only add if this spot has not been added before
                if !positions.contains(&new_pos) && let Some(row) = follow.get(new_pos.y as usize) &&
                    let Some(cc) = row.chars().nth(new_pos.x as usize) && cc == 'O' {
                    out_path.push(PathNode::new(SpatialAction::new(new_pos, *m)));
                    positions.push(current);
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

    fn draw_path(&self, path: &Vec<PathNode<Vector2i>>) -> String {
        let mut environ_str: Vec<String> = self.environment.iter()
            .map(|row| row.to_string()).collect();

        // iterate through the positions
        for i in 0..path.len() {
            let pos = path[i].action.pos;
            let c;

            c = match path[i].action.move_action {
                Some(came_from) => {
                    if let Some(going_to_pn) = path.get(i + 1) &&
                        let Some(going_to) = going_to_pn.action.move_action {
                        match came_from.offset {
                            vec2i!(0, 1) => match going_to.offset {
                                vec2i!(0, 1) => '↑',
                                vec2i!(0, -1) => '⤓',
                                vec2i!(1, 0) => '↱',
                                vec2i!(-1, 0) => '↰',
                                _ => '?'
                            },
                            vec2i!(0, -1) => match going_to.offset {
                                vec2i!(0, 1) => '⤒',
                                vec2i!(0, -1) => '↓',
                                vec2i!(1, 0) => '↳',
                                vec2i!(-1, 0) => '↲',
                                _ => '?'
                            },
                            vec2i!(1, 0) => match going_to.offset {
                                vec2i!(0, 1) => '⬏',
                                vec2i!(0, -1) => '⬎',
                                vec2i!(1, 0) => '→',
                                vec2i!(-1, 0) => '⇤',
                                _ => '?'
                            },
                            vec2i!(-1, 0) => match going_to.offset {
                                vec2i!(0, 1) => '⬐',
                                vec2i!(0, -1) => '⬑',
                                vec2i!(1, 0) => '⇥',
                                vec2i!(-1, 0) => '←',
                                _ => '?'
                            },
                            _ => '?'
                        }
                    } else {
                        'G'
                    }
                }
                None => '@'
            };

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
        assert!(out.is_ok(), "Pathfinding failed with error: {}", out.unwrap_err());
        let path = out.unwrap();
        assert!(path.len() > 0, "Pathfinder returned an empty path");
        assert_eq!(path.last().unwrap().action.pos, end, "Pathfinder did not reach the end successfully");

        println!("---SUCCESS---\n{}", self.draw_path(&path));
        self.calc.reset()
    }

    fn eval_failure(&mut self, start: Vector2i, end: Vector2i) {
        let out = self.calc.calculate(start, end);
        assert!(out.is_ok(), "Pathfinding failed with error: {}", out.unwrap_err());
        assert_eq!(out.unwrap().len(), 0, "Pathfinder calculated an impossible path");
        self.calc.reset();
    }
}