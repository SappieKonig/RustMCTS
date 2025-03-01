use crate::games::connect4::board::Board;

#[derive(Clone, Copy, Debug)]
pub struct BitBoard {
    player1: u64,   // Current player's pieces
    player2: u64,  // Opponent's pieces
    heights: [u8; 7], // Height of each column (0-5)
    current_player: u8,
}

const WINNING_MASKS: [[u64; 13]; 42] = generate_winning_masks();
const MASK_COUNTS: [u8; 42] = generate_mask_counts();

const fn generate_mask_counts() -> [u8; 42] {
    let mut counts = [0; 42];
    
    let mut row = 0;
    while row < 6 {
        let mut col = 0;
        while col < 7 {
            let pos = row * 7 + col;
            let mut count = 0;
            
            // Horizontal patterns
            if col <= 3 { count += 1; }  // As leftmost
            if col >= 1 && col <= 4 { count += 1; }  // As second
            if col >= 2 && col <= 5 { count += 1; }  // As third
            if col >= 3 { count += 1; }  // As rightmost
            
            // Vertical patterns (board height limits to 3 patterns)
            if row <= 2 { count += 1; }  // As bottom
            if row >= 1 && row <= 3 { count += 1; }  // As second
            if row >= 2 && row <= 4 { count += 1; }  // As third
            if row >= 3 { count += 1; }  // As top
            
            // Diagonal up-right
            if row <= 2 && col <= 3 { count += 1; }  // As bottom-left
            if row >= 1 && row <= 3 && col >= 1 && col <= 4 { count += 1; }  // As second
            if row >= 2 && row <= 4 && col >= 2 && col <= 5 { count += 1; }  // As third
            if row >= 3 && col >= 3 { count += 1; }  // As top-right

            // Diagonal up-left
            if row <= 2 && col >= 3 { count += 1; }  // As bottom-right
            if row >= 1 && row <= 3 && col >= 2 && col <= 5 { count += 1; }  // As second
            if row >= 2 && row <= 4 && col >= 1 && col <= 4 { count += 1; }  // As third
            if row >= 3 && col <= 3 { count += 1; }  // As top-left
            
            counts[pos] = count;
            col += 1;
        }
        row += 1;
    }
    
    counts
}

const fn generate_winning_masks() -> [[u64; 13]; 42] {
    let mut masks = [[0; 13]; 42];
    
    let mut row = 0;
    while row < 6 {
        let mut col = 0;
        while col < 7 {
            let pos = row * 7 + col;
            let mut mask_idx = 0;

            // Horizontal patterns
            if col <= 3 {
                // As leftmost piece
                masks[pos][mask_idx] = 0b1111u64 << (row * 7 + col);
                mask_idx += 1;
            }
            if col >= 1 && col <= 4 {
                // As second piece
                masks[pos][mask_idx] = 0b1111u64 << (row * 7 + col - 1);
                mask_idx += 1;
            }
            if col >= 2 && col <= 5 {
                // As third piece
                masks[pos][mask_idx] = 0b1111u64 << (row * 7 + col - 2);
                mask_idx += 1;
            }
            if col >= 3 {
                // As rightmost piece
                masks[pos][mask_idx] = 0b1111u64 << (row * 7 + col - 3);
                mask_idx += 1;
            }

            // Vertical patterns
            if row <= 2 {
                // As bottom piece
                let mut vmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    vmask |= 1u64 << ((row + r) * 7 + col);
                    r += 1;
                }
                masks[pos][mask_idx] = vmask;
                mask_idx += 1;
            }
            if row >= 1 && row <= 3 {
                // As second piece
                let mut vmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    vmask |= 1u64 << ((row - 1 + r) * 7 + col);
                    r += 1;
                }
                masks[pos][mask_idx] = vmask;
                mask_idx += 1;
            }
            if row >= 2 && row <= 4 {
                // As third piece
                let mut vmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    vmask |= 1u64 << ((row - 2 + r) * 7 + col);
                    r += 1;
                }
                masks[pos][mask_idx] = vmask;
                mask_idx += 1;
            }

            if row >= 3 {
                // As top piece
                let mut vmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    vmask |= 1u64 << ((row - 3 + r) * 7 + col);
                    r += 1;
                }
                masks[pos][mask_idx] = vmask;
                mask_idx += 1;
            }

            // Diagonal up-right patterns
            if row <= 2 && col <= 3 {
                // As bottom-left piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row + r) * 7 + (col + r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }
            if row >= 1 && row <= 3 && col >= 1 && col <= 4 {
                // As second piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row - 1 + r) * 7 + (col - 1 + r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }
            if row >= 2 && row <= 4 && col >= 2 && col <= 5 {
                // As third piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row - 2 + r) * 7 + (col - 2 + r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }
            if row >= 3 && col >= 3 {
                // As top-right piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row - 3 + r) * 7 + (col - 3 + r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }

            // Diagonal up-left patterns
            if row <= 2 && col >= 3 {
                // As bottom-right piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row + r) * 7 + (col - r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }
            if row >= 1 && row <= 3 && col >= 2 && col <= 5 {
                // As second piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row - 1 + r) * 7 + (col + 1 - r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }
            if row >= 2 && row <= 4 && col >= 1 && col <= 4 {
                // As third piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row - 2 + r) * 7 + (col + 2 - r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }
            if row >= 3 && col <= 3 {
                // As top-left piece
                let mut dmask = 0u64;
                let mut r = 0;
                while r < 4 {
                    dmask |= 1u64 << ((row - 3 + r) * 7 + (col + 3 - r));
                    r += 1;
                }
                masks[pos][mask_idx] = dmask;
                mask_idx += 1;
            }

            // Fill remaining slots with 0s
            while mask_idx < 13 {
                masks[pos][mask_idx] = 0;
                mask_idx += 1;
            }

            col += 1;
        }
        row += 1;
    }
    
    masks
}

impl BitBoard {
    pub fn from_board(board: &Board, current_player: u8) -> Self {
        let mut player1 = 0u64;
        let mut player2 = 0u64;
        let mut heights = [0u8; 7];

        for col in 0..crate::config::GRID_COLS {
            for row in 0..crate::config::GRID_ROWS {
                let pos = row * crate::config::GRID_COLS + col;
                match board.cells[row][col] {
                    1 => player1 |= 1u64 << pos,
                    2 => player2 |= 1u64 << pos,
                    _ => {}
                }
                if board.cells[row][col] != 0 {
                    heights[col] = (row + 1) as u8; // Height is 1-based
                }
            }
        }

        Self {
            player1,
            player2,
            heights,
            current_player,
        }
    }

    pub fn is_valid_move(&self, col: usize) -> bool {
        self.heights[col] < 6
    }

    pub fn get_current_player(&self) -> u8 {
        self.current_player
    }

    pub fn make_move(&mut self, col: usize) -> Option<i32> {
        let row = self.heights[col];
        let pos = row as u64 * 7 + col as u64;
        if self.current_player == 1 {
            self.player1 |= 1u64 << pos;
        } else {
            self.player2 |= 1u64 << pos;
        }
        let is_winning = self.is_winning_move(col);
        self.heights[col] += 1;
        let mut res: Option<i32> = None;
        if is_winning {
            res = Some(3 - 2 * self.current_player as i32);
        } else if self.is_full() {  
            res = Some(0);
        }
        self.current_player = 3 - self.current_player;
        res
    }

    fn is_winning_move(&self, col: usize) -> bool {
        let row = self.heights[col] as usize;
        let pos = row * 7 + col;
        let board = if self.current_player == 1 {
            self.player1 | (1u64 << pos)
        } else {
            self.player2 | (1u64 << pos)
        };

        let num_patterns = MASK_COUNTS[pos] as usize;
        for i in 0..num_patterns {
            let mask = WINNING_MASKS[pos][i];
            if (board & mask) == mask {
                return true;
            }
        }
        false
    }

    pub fn get_valid_moves(&self) -> Vec<usize> {
        (0..7).filter(|&col| self.is_valid_move(col)).collect()
    }

    pub fn is_full(&self) -> bool {
        self.heights.iter().all(|&h| h >= 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = BitBoard::new();
        assert_eq!(board.current, 0);
        assert_eq!(board.opponent, 0);
        assert_eq!(board.heights, [0; 7]);
    }

    #[test]
    fn test_valid_moves() {
        let mut board = BitBoard::new();
        assert!(board.is_valid_move(0));
        
        // Fill up column 0
        for _ in 0..6 {
            assert!(board.make_move(0));
        }
        
        // Column should now be full
        assert!(!board.is_valid_move(0));
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = BitBoard::new();
        
        // Player 1: 0,1,2
        board.make_move(0);
        board.make_move(1);
        board.make_move(2);
        
        // Player 2: 6,6,6
        board.make_move(6);
        board.make_move(6);
        board.make_move(6);
        
        // Test winning move for Player 1
        assert!(board.is_winning_move(3));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = BitBoard::new();
        
        // Player 1: 0,0,0
        board.make_move(0);
        board.make_move(0);
        board.make_move(0);
        
        // Player 2: 1,1,1
        board.make_move(1);
        board.make_move(1);
        board.make_move(1);
        
        // Test winning move for Player 1
        assert!(board.is_winning_move(0));
    }

    #[test]
    fn test_winning_mask_counts() {
        // Test corners (should have fewer patterns)
        assert_eq!(MASK_COUNTS[0], 3);  // Bottom-left corner: horizontal(1) + vertical(1) + diagonal-up-right(1)
        assert_eq!(MASK_COUNTS[1], 4);  // Bottom-left corner: horizontal(2) + vertical(1) + diagonal-up-right(1)
        assert_eq!(MASK_COUNTS[6], 3);  // Bottom-right corner: horizontal(1) + vertical(1)
        assert_eq!(MASK_COUNTS[35], 3); // Top-left corner: horizontal(1) + vertical(1) + diagonal-up-left(1)
        assert_eq!(MASK_COUNTS[41], 3); // Top-right corner: horizontal(1) + vertical(1) + diagonal-up-right(1)

        // Test central positions (should have more patterns)
        let center_pos = 3 * 7 + 3;  // Position (3,3)
        assert_eq!(MASK_COUNTS[center_pos], 13); // Should have maximum patterns
    }
}