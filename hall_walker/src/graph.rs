#[derive(Debug, Clone, Copy)]
pub struct Neighbors {
    edges: u8
}

pub struct Graph {
    graph: Vec<Neighbors>,
    dim: u8

}

impl Neighbors {
    pub fn new() -> Self {
        Self {
            edges: 0
        }
    }
}

impl Graph {
    pub fn new(dim: usize) -> Self {
        let mut v = Vec::new();
        let mut tmp: Neighbors = Neighbors { edges: 0 };
        for i in 0..81 {
            let mut t = tmp.clone();
            v.push(t);
        }
        Self {
            graph: v,
            dim: dim as u8
        }
    }

    pub fn get(&mut self, x: u8, y: u8) -> () {
        self.graph.get((x * self.dim + y) as usize);
    }

    // pub fn neighbors()
}


pub static BOARD_WIDTH: i64 = 9;