use algorithms::a_star::{a_star, Position, Terrain};

fn main() {
    // Exemplo de grid 2D com diferentes tipos de terreno
    let grid = vec![
        vec![
            Terrain::Plain,
            Terrain::Forest,
            Terrain::Mountain,
            Terrain::Plain,
        ],
        vec![
            Terrain::Plain,
            Terrain::Water,
            Terrain::Forest,
            Terrain::Mountain,
        ],
        vec![
            Terrain::Mountain,
            Terrain::Plain,
            Terrain::Forest,
            Terrain::Plain,
        ],
        vec![
            Terrain::Water,
            Terrain::Plain,
            Terrain::Mountain,
            Terrain::Plain,
        ],
    ];

    // Ponto de início e objetivo
    let start = Position { row: 0, col: 0 };
    let goal = Position { row: 3, col: 3 };

    // Executando o algoritmo A*
    if let Some((cost, path)) = a_star(&grid, start.clone(), goal.clone()) {
        println!("Custo total: {}", cost);
        println!("Caminho:");
        for pos in path {
            println!("({}, {})", pos.row, pos.col);
        }
    } else {
        println!("Nenhum caminho encontrado.");
    }
}
