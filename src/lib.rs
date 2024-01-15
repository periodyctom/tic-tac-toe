use std::fmt::{Display, Formatter, Write};

const WIDTH: usize = 3usize;
const VERTICAL_DECO: &str = "-|0|1|2|-";

#[derive(PartialEq, Copy, Clone)]
pub struct CellIndex(pub u8, pub u8);

pub enum TurnResult {
    Continue,
    Draw,
    Winner(Symbol),
    InvalidMove,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Symbol {
    X,
    O,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::X => f.write_char('X'),
            Symbol::O => f.write_char('O'),
        }
    }
}

pub struct Board {
    cells: [Option<Symbol>; WIDTH * WIDTH],
    current_player: Symbol,
}

impl Board {
    pub fn new(initial_player: Symbol) -> Board {
        Board {
            cells: [None; 9],
            current_player: initial_player,
        }
    }

    pub fn current_player(&self) -> Symbol {
        self.current_player
    }

    fn check_for_draw(&self) -> bool {
        self.cells.iter().all(|&x| x != None)
    }

    pub fn get(&self, index: CellIndex) -> Option<Symbol> {
        let offset: usize = index.0 as usize + WIDTH * index.1 as usize;
        self.cells[offset]
    }

    fn set(&mut self, index: CellIndex, symbol: Symbol) {
        let offset: usize = index.0 as usize + WIDTH * index.1 as usize;
        self.cells[offset] = Some(symbol);
    }

    fn get_symbol(symbol: Option<Symbol>) -> char {
        match symbol {
            None => ' ',
            Some(Symbol::X) => 'X',
            Some(Symbol::O) => 'O',
        }
    }

    fn write_row(&self, output: &mut String, row_index: u8) {
        let row_str = format!("{row_index}");

        output.push_str(row_str.as_str());
        output.push('|');

        for column in 0..=2 {
            let cell = self.get(CellIndex(column, row_index));
            let cell_symbol_str = Self::get_symbol(cell);
            let cell_str = format!("{cell_symbol_str}|");
            output.push_str(cell_str.as_str());
        }

        output.push_str(row_str.as_str());
        output.push_str("\n");
    }

    pub fn as_text(&self) -> String {
        let mut output = String::from(VERTICAL_DECO);
        output.push('\n');
        self.write_row(&mut output, 0u8);
        self.write_row(&mut output, 1u8);
        self.write_row(&mut output, 2u8);
        output.push_str(VERTICAL_DECO);
        output.push('\n');
        output
    }

    pub fn process_turn(&mut self, index: CellIndex) -> TurnResult {
        let successful_move = self.try_player_move(index);

        if !successful_move {
            return TurnResult::InvalidMove;
        }

        if let Some(win_line) = self.check_for_win() {
            return TurnResult::Winner(win_line);
        }

        if self.check_for_draw() {
            return TurnResult::Draw;
        }

        self.next_player_turn();
        TurnResult::Continue
    }

    fn try_player_move(&mut self, index: CellIndex) -> bool {
        match self.get(index) {
            None => {
                self.set(index, self.current_player);
                true
            }
            Some(_) => false,
        }
    }

    fn next_player_turn(&mut self) {
        match self.current_player {
            Symbol::X => self.current_player = Symbol::O,
            Symbol::O => self.current_player = Symbol::X,
        }
    }

    fn check_for_win(&self) -> Option<Symbol> {
        if let Some(row) = self.check_rows() {
            return Some(row);
        } else if let Some(column) = self.check_columns() {
            return Some(column);
        } else if let Some(diagonal) = self.check_diagonals() {
            return Some(diagonal);
        }
        None
    }

    fn check_rows(&self) -> Option<Symbol> {
        self.check_win_line(true)
    }

    fn check_columns(&self) -> Option<Symbol> {
        self.check_win_line(false)
    }

    fn check_diagonals(&self) -> Option<Symbol> {
        let center_index = CellIndex(1, 1);
        let center = self.get(center_index);

        if center != None {
            let top_left_index = CellIndex(0, 0);
            let top_right_index = CellIndex(2, 0);

            let bottom_left_index = CellIndex(0, 2);
            let bottom_right_index = CellIndex(2, 2);

            let bottom_left = self.get(bottom_left_index);
            let bottom_right = self.get(bottom_right_index);

            let top_left = self.get(top_left_index);
            let top_right = self.get(top_right_index);

            if (top_left == center) && (center == bottom_right)
                || (bottom_left == center) && (center == top_right)
            {
                return center;
            }
        }

        None
    }

    fn check_win_line(&self, is_rows: bool) -> Option<Symbol> {
        for i in 0..=2 {
            let a = self.get(Self::select_index(0, i, is_rows));
            let b = self.get(Self::select_index(1, i, is_rows));
            let c = self.get(Self::select_index(2, i, is_rows));

            if a != None && (a == b) && (b == c) {
                return a;
            }
        }

        None
    }

    fn select_index(a: u8, b: u8, is_rows: bool) -> CellIndex {
        let x = Self::select(a, b, is_rows);
        let y = Self::select(b, a, is_rows);

        CellIndex(x, y)
    }

    fn select(x: u8, y: u8, selector: bool) -> u8 {
        if selector {
            x
        } else {
            y
        }
    }
}
