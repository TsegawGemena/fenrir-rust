use super::transition::{DecodeState, TransitionTable};

#[derive(Debug, Clone)]
pub struct StateGraph {
    nodes: Vec<DecodeState>,
    edges: Vec<(DecodeState, DecodeState)>,
}

impl StateGraph {
    pub fn from_table(table: &TransitionTable) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for t in table.iter() {
            if !nodes.contains(&t.from) { nodes.push(t.from); }
            if !nodes.contains(&t.to) { nodes.push(t.to); }
            edges.push((t.from, t.to));
        }
        Self { nodes, edges }
    }

    pub fn reachable_from(&self, start: DecodeState) -> Vec<DecodeState> {
        let mut seen = vec![start];
        let mut queue = vec![start];
        while let Some(cur) = queue.pop() {
            for (from, to) in &self.edges {
                if *from == cur && !seen.contains(to) {
                    seen.push(*to);
                    queue.push(*to);
                }
            }
        }
        seen
    }
}
impl StateGraph {
    pub fn edge_count_at_0(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(0 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_1(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(1 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_2(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(2 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_3(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(3 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_4(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(4 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_5(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(5 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_6(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(6 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_7(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(7 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_8(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(8 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_9(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(9 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_10(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(10 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_11(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(11 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_12(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(12 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_13(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(13 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_14(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(14 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_15(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(15 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_16(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(16 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_17(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(17 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_18(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(18 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_19(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(19 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_20(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(20 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_21(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(21 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_22(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(22 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_23(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(23 % 3)
    }
}
impl StateGraph {
    pub fn edge_count_at_24(&self, state: DecodeState) -> usize {
        self.edges.iter().filter(|(f, _)| *f == state).count().saturating_add(24 % 3)
    }
}
