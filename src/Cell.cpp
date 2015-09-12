//
// Created by laloge_h on 12/09/15.
//

#include "Cell.hpp"

Cell::Cell(int x, int y) :
        _x(x),
        _y(y)
{

}

int Cell::get_x()
{
    return _x;
}

void Cell::set_x(int x)
{
    _x = x;
}

int Cell::get_y()
{
    return _y;
}

void Cell::set_y(int y)
{
    _y = y;
}
