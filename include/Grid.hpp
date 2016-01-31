//
// Created by laloge_h on 13/09/15.
//

#ifndef TIG_100_GRID_HPP
#define TIG_100_GRID_HPP

#include <array>
#include "Cell.hpp"

#define GRID_HEIGHT 3
#define GRID_WIDTH  4

class Grid
{
protected:
  std::array<std::array<Cell*, GRID_WIDTH>, GRID_HEIGHT> _grid;

public:
  Grid();

  void draw() const;
};


#endif //TIG_100_GRID_HPP
