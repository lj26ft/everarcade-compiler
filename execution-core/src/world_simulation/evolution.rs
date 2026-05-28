use super::terrain::TerrainCell;
pub fn evolve_cell(cell: &mut TerrainCell, tick: u64) {
    cell.height += ((cell.x as i64 * 31 + cell.y as i64 * 17 + tick as i64) % 3) - 1;
}
