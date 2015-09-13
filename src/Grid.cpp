//
// Created by laloge_h on 13/09/15.
//

#include <ncursesw/curses.h>
#include "CodeCell.hpp"
#include "Grid.hpp"

Grid::Grid()
{
	for (unsigned int y(0); y < GRID_HEIGHT; y++)
	{
		for (unsigned int x(0); x < GRID_WIDTH; x++)
		{
			_grid[y][x] = new CodeCell;
		}
	}
}

void Grid::draw() const
{
	for (unsigned int y(0); y < GRID_HEIGHT; y++)
	{
		for (unsigned int x(0); x < GRID_WIDTH; x++)
		{
			move(y * (CELL_HEIGHT + 1) + 2, x * (CELL_WIDTH + 5));
			_grid[y][x]->draw();
		}
	}
	refresh();
}
