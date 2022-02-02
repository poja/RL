use petgraph::{
    data::Build,
    graphmap::{DiGraphMap, GraphMap, NodeIdentifiers}, matrix_graph::NodeIdentifiers,
};

use crate::hex_game::{HexPlayer, HexPosition, Location};

/// Monte Carlo Tree Search implementation
/// TODO make this general and not Hex-specific

struct MCTSNode {
    state: HexPosition,

    /// This is the variable n from UCT formula
    simulations_n: i32,

    /// This is the variable w from UCT formula
    wins_w: i32,
}

impl MCTSNode {
    pub fn from_position(pos: HexPosition) -> Self {
        Self {
            state: pos,
            simulations_n: 0,
            wins_w: 0,
        }
    }
}

struct MCTSPlayer {
    game_tree: DiGraphMap<MCTSNode, i32>,
    tree_root: NodeIdentifiers<>,
    exploration_parameter_c: f32,
    simulations_per_move: i32,
}

impl MCTSPlayer {
    pub fn new() -> Self {
        Self {
            game_tree: DiGraphMap::new(),
            tree_root: None,
            exploration_parameter_c: (2 as f32).sqrt(),
            simulations_per_move: 10000,
        }
    }

    pub fn develop_tree(&mut self, position: &HexPosition) -> () {
        assert!(self.game_tree.edge_count() == 0);
        self.tree_root = game_tree.add_node(MCTSNode::from_position(position.clone()));
        for i in 1..self.simulations_per_move {
            println!("Simulating: ({}/{})", i, self.simulations_per_move);
            let selected = self.select_node();
            let game_outcome: i32 = self.simulate_playout(&selected.state);
            self.backpropagate(selected, game_outcome);
        }
    }

    fn select_node(&mut self) -> NodeIdentifiers {
        self.tree_root.unwrap() // TODO
    }

    /// Returns 1 if starting player won, -1 if lost, 0 if draw
    fn simulate_playout(&self, state: &HexPosition) -> i32 {
        0 // actually play game
    }

    fn backpropagate(&self, node: NodeIdentifiers, game_outcome: i32) -> () {
        // TODO
    }

    pub fn current_best_move(&self) -> Location {
        (0, 0) // TODO search the tree
    }
}

impl HexPlayer for MCTSPlayer {
    fn next_move(&mut self, position: &HexPosition) -> Location {
        self.develop_tree(position);
        self.current_best_move()
    }
}
