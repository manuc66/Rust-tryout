
#[test]
fn a_new_cell_have_no_value() {
	let cell = Cell::new();
	assert!(cell.get_value().is_none());
}

#[test]
fn a_new_cell_have_6_possible_values() {
	let cell = Cell::new();
	assert_eq!(cell.how_many_possible(), 6);
}

#[test]
fn by_default_anything_is_possible_in_a_cell() {
	let cell = Cell::new();
	assert!(cell.is_possible_value(1));
	assert!(cell.is_possible_value(2));
	assert!(cell.is_possible_value(3));
	assert!(cell.is_possible_value(4));
	assert!(cell.is_possible_value(5));
	assert!(cell.is_possible_value(6));
}

#[test]
fn a_cell_keeps_track_of_which_values_are_possible() {
	let mut cell = Cell::new();
	cell.not_possible(3);
	cell.not_possible(4);

	assert_eq!(cell.how_many_possible(), 4);
	assert_eq!(cell.get_value().is_none(), true);
	assert_eq!(cell.is_possible_value(5), true);
	assert_eq!(cell.is_possible_value(3), false);
}
#[test]
fn if_only_one_possiblity_is_left_then_a_cell_knows_its_value() {
	let mut cell = Cell::new();
	cell.not_possible(1);
	cell.not_possible(2);
	
	cell.not_possible(4);
	cell.not_possible(5);
	cell.not_possible(6);


	assert_eq!(cell.how_many_possible(), 1);
	assert_eq!(cell.get_value().unwrap(), 3) ;
	assert_eq!(cell.is_possible_value(4), false);
	assert_eq!(cell.is_possible_value(2), false);
}


	pub struct Cell {
		value: Option<int>,
		possible_values:     Vec<int>,
	}
	impl Cell {
		pub fn new() -> Cell { 
			Cell { 
				value : None, 
				possible_values : Vec::from_fn(6, |idx| (idx + 1).to_int().unwrap()) }
		}

		pub fn get_value(&self) -> Option<int> {
			self.value	
		}

		pub fn how_many_possible(&self) -> uint {
			self.possible_values.len()
		}		
	
		pub fn not_possible(&mut self, value: int) {
			self.possible_values.retain(|elem| *elem != value);
			if self.how_many_possible() == 1 {
				self.value = Some(self.possible_values[0]);			
			}
		}

		pub fn is_possible_value(&self, value: int) -> bool {
			self.possible_values.iter().any(|elem| *elem == value)
		}
	}

#[test]
fn a_grid_have_a_name() {
	let grid = Grid::new("B");
	assert_eq!(grid.name, "B");
}

#[test]
fn a_grid_consists_of_3by2_cells() {
	let grid = Grid::new("A");
	assert_eq!(type_name_of(&grid.cells) , "[[Cell, ..2], ..3]");
}

pub struct Grid {
	pub name: &'static str,
	pub cells: [[Cell, ..2], ..3],
}
impl Grid {
	pub fn new(name : &'static str) -> Grid { 
		Grid { name : name, cells : 
			[
				[Cell::new(),Cell::new()],
				[Cell::new(),Cell::new()],
				[Cell::new(),Cell::new()],	
			]
		}
	}
}

fn type_name_of<T>(_: &T) -> &'static str { 
	unsafe { (*std::intrinsics::get_tydesc::<T>()).name } 
}
	
