use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// Enum para diferentes tipos de terreno
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Terrain {
    Plain,    // Terreno plano, transição normal
    Forest,   // Floresta, aumenta o custo de transição
    Mountain, // Montanha, custo de transição muito alto
    Water,    // Água, permite transição apenas com um custo especial
}

impl Terrain {
    // Método para determinar o custo base do terreno
    fn base_cost(&self) -> usize {
        match self {
            Terrain::Plain => 1,
            Terrain::Forest => 3,
            Terrain::Mountain => 5,
            Terrain::Water => 10,
        }
    }

    // Método para calcular o custo de transição entre dois terrenos
    fn transition_cost(from: &Terrain, to: &Terrain) -> usize {
        match (from, to) {
            (Terrain::Plain, Terrain::Plain) => 1,
            (Terrain::Plain, Terrain::Forest) => 2,
            (Terrain::Plain, Terrain::Mountain) => 4,
            (Terrain::Forest, Terrain::Plain) => 2,
            (Terrain::Forest, Terrain::Forest) => 3,
            (Terrain::Forest, Terrain::Mountain) => 5,
            (Terrain::Mountain, Terrain::Plain) => 4,
            (Terrain::Mountain, Terrain::Forest) => 5,
            (Terrain::Mountain, Terrain::Mountain) => 6,
            (Terrain::Water, _) => 10, // Transições de/para água têm custo alto
            (_, Terrain::Water) => 10,
        }
    }
}

// Representação das coordenadas de uma célula
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

// Definição de um estado com seu custo
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
    terrain: Terrain,
}

// Para usar na min-heap, ordenando pelo menor custo
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // menor custo tem maior prioridade
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Função heurística: distância de Manhattan
fn manhattan_distance(a: &Position, b: &Position) -> usize {
    (a.row as isize - b.row as isize).unsigned_abs()
        + (a.col as isize - b.col as isize).unsigned_abs()
}

// Algoritmo A* para encontrar o caminho mais curto no grid
pub fn a_star(
    grid: &Vec<Vec<Terrain>>,
    start: Position,
    goal: Position,
) -> Option<(usize, Vec<Position>)> {
    let directions = vec![
        (0, 1),  // direita
        (0, -1), // esquerda
        (1, 0),  // baixo
        (-1, 0), // cima
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut heap = BinaryHeap::new();
    let mut g_costs = vec![vec![usize::MAX; cols]; rows];
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    g_costs[start.row][start.col] = 0; // custo inicial é zero
    heap.push(State {
        cost: 0,
        position: start.clone(),
        terrain: grid[start.row][start.col].clone(),
    });

    while let Some(State {
        cost,
        position,
        terrain,
    }) = heap.pop()
    {
        // Se alcançarmos o objetivo, construir o caminho
        if position == goal {
            let mut path = Vec::new();
            let mut current = position.clone();
            while current != start {
                path.push(current.clone());
                current = came_from[&current].clone();
            }
            path.push(start);
            path.reverse();
            return Some((cost, path));
        }

        // Explorar vizinhos
        for &(dr, dc) in &directions {
            let new_row = position.row as isize + dr;
            let new_col = position.col as isize + dc;

            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                let new_pos = Position {
                    row: new_row as usize,
                    col: new_col as usize,
                };

                let next_terrain = &grid[new_pos.row][new_pos.col];
                let transition_cost = Terrain::transition_cost(&terrain, next_terrain);

                let new_cost = cost + next_terrain.base_cost() + transition_cost;

                if new_cost < g_costs[new_pos.row][new_pos.col] {
                    g_costs[new_pos.row][new_pos.col] = new_cost;
                    let f_cost = new_cost + manhattan_distance(&new_pos, &goal); // f(n) = g(n) + h(n)
                    heap.push(State {
                        cost: f_cost,
                        position: new_pos.clone(),
                        terrain: next_terrain.clone(),
                    });
                    came_from.insert(new_pos, position.clone());
                }
            }
        }
    }

    None // Nenhum caminho encontrado
}
