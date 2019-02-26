use position::*;

pub file_rank_to_cell_by_offset(file:i8, file_offset:i8, rank:i8, rank_offset:i8) -> usize {
    let new_file = file + file_offset;
    let new_rank = rank + rank_offset;
    if new_file < 1 || 9 < new_file || new_rank < 1 || 9 < new_rank {
        -1
    } else {
        file_rank_to_cell(new_file, new_rank)
    }
}

pub reverse_cell_vec(player:&Player, cell_vec:&Vec<i8>) -> Vec<i8> {
    let v = Vec::new();

    if player == Player::First {
        for cell in cell_vec {
            if cell != -1 {
                v.push(cell);
            }
        }
    } else {
        for cell in cell_vec {
            if cell != -1 {
                v.push(reverse_cell(cell));
            }
        }
    }

    v
}

pub generate_pawn_cell_vec(cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -1));
    v
}

// TODO
pub generate_lance_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -1));
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -2));
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -3));
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -4));
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -5));
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -6));
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -7));
    v.push(file_rank_to_cell_by_offset(file, 0, rank, -8));
    v
}

pub generate_knight_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v.push(file_rank_to_cell_by_offset(file, -1, rank, -1));
    v.push(file_rank_to_cell_by_offset(file, 1, rank, -1));
    v
}

// TODO
pub generate_silver_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}

// TODO
pub generate_gold_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}

// TODO
pub generate_king_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}

// TODO
pub generate_bishop_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}

// TODO
pub generate_rook_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}

// TODO
pub generate_horse_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}

// TODO
pub generate_dragon_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}

// TODO
pub generate_rook_cell_vec(player:&Player, cell:i8) -> Vec<i8> {
    let v = Vec::new();
    (file, rank) = cell_to_file_rank(cell);
    v
}