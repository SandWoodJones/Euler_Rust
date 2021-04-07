/* Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there
 * are exactly 6 routes to the bottom right corner. How many such routes are there through a 20×20 grid? */

use crate::my_math::bin_coeff;

pub fn answer() -> u64 {
	let width = 20;
	let height = 20;
	
	bin_coeff(width + height, width)
}
