//
// Created by laloge_h on 13/09/15.
//

#include <curses.h>
#include "CodeCell.hpp"
#include "Grid.hpp"

#define CELL_X_DISTANCE 5
#define CELL_Y_DISTANCE 1
#define CELL_X_OFFSET	66
#define CELL_Y_OFFSET	1

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
      move(y * (CELL_HEIGHT + CELL_Y_DISTANCE) + CELL_Y_OFFSET,
	   x * (CELL_WIDTH + CELL_X_DISTANCE) + CELL_X_OFFSET);
      _grid[y][x]->draw();
    }
  }
  refresh();
}
