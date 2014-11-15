#![allow(dead_code, unused_imports, unused_variables)]

use std::vec::Vec;
use std::option::Option;

enum Cell {
    A,
    D
}

struct World {
    width: uint,
    height: uint,
    state: Vec<Cell>
}

impl World {

    fn try_create(width: uint, height: uint, state: Vec<Cell>) -> Option<World> {
        if width * height != state.len() {
            None
        }
        else {
            Some(World { width: width, height: height, state: Vec::new() })
        }
    }

    enum Offset {
        Less,
        Same,
        More
    }

    fn get_actual_index(max: uint, current_index: uint, offset: &Offset) {
        match *offset {
            Less => if current_index == 0 { max - 1 } else { current_index - 1 },
            Same => current_index,
            More => if current_index >= (max - 1) { 0 } else { current_index + 1 }
        }
    }

    fn find_neighbours(&self, row: uint, cell: uint) -> u8 {
        
        let mut neighbours = 0;

        for row_offset in vec![Less, Same, More].iter() {

            let row_actual = get_actual_index(self.height, row, row_offset); 

            for cell_offset in vec![Less, Same, More].iter() {

                let row_actual = get_actual_index(self.width, cell, cell_offset); 

                let neighbour_is_alive = match self.state[row_actual * self.height + cell_actual] {
                    A => true,
                    D => false
                };

                if neighbour_is_alive {
                    neighbours += 1;
                }
            }
        }

        neighbours
    }
}



mod test {

    use super::{ World, Cell, A, D };

    #[test]
    fn math_checks_out() {
        assert_eq!(25i, 5i * 5i);
    }

    #[test]
    fn can_create_world() {
        
        let state = Vec::from_fn(100, |_| D);

        let w = World::try_create(10, 10, state);

        assert!(w.is_some());
    }

    #[test]
    fn can_fail_to_create_world() {
        
        let state = Vec::from_fn(99, |_| D);

        let w = World::try_create(10, 10, state);

        assert!(w.is_none());
    }

    fn make_square_board() -> World {
        let state = vec![
            A, A, A,
            A, D, A,
            A, A, A,
        ];
        World::try_create(3, 3, state).expect("Invalid world")
    }

    #[test]
    fn can_count_neighbours_on_square_board() {

        let w = make_square_board();

        let neighbours = w.find_neighbours(1, 1);

        assert_eq!(neighbours, 8);
    }
}