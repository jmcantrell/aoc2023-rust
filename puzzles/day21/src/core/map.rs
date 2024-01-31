use super::{Direction, Graph, Grid, Location};

fn has_garden_at(grid: &Grid, location: Location<usize>) -> bool {
    grid.get(location).is_some_and(|&tile| tile.is_garden())
}

#[derive(Debug, Clone)]
pub struct MapWalled {
    pub grid: Grid,
    pub start: Location<usize>,
}

impl Graph for MapWalled {
    type Component = usize;

    fn start(&self) -> Location<Self::Component> {
        self.start
    }

    fn neighbor(
        &self,
        (row, column): Location<Self::Component>,
        direction: Direction,
    ) -> Option<Location<Self::Component>> {
        let (vertical, horizontal) = direction.step();

        let adjacent = (
            row.checked_add_signed(vertical)?,
            column.checked_add_signed(horizontal)?,
        );

        has_garden_at(&self.grid, adjacent).then_some(adjacent)
    }
}

#[derive(Debug, Clone)]
pub struct MapInfinite {
    pub grid: Grid,
    pub start: Location<isize>,
}

impl Graph for MapInfinite {
    type Component = isize;

    fn start(&self) -> Location<Self::Component> {
        self.start
    }

    fn neighbor(
        &self,
        (row, column): Location<Self::Component>,
        direction: Direction,
    ) -> Option<Location<Self::Component>> {
        let (height, width) = self.grid.shape();
        let (vertical, horizontal) = direction.step();

        let row = row.checked_add(vertical)?;
        let column = column.checked_add(horizontal)?;

        has_garden_at(
            &self.grid,
            (
                row.rem_euclid(height as isize) as usize,
                column.rem_euclid(width as isize) as usize,
            ),
        )
        .then_some((row, column))
    }
}
