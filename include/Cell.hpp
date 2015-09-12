//
// Created by laloge_h on 12/09/15.
//

#ifndef TIG_100_CELL_HPP
#define TIG_100_CELL_HPP

/**
 * Basic cell
 */
class Cell
{
protected:
    int _x;
    int _y;

public:
    Cell(int x, int y);

    virtual void draw() = 0;

    int get_x();
    int get_y();

    void set_x(int x);
    void set_y(int y);
};


#endif //TIG_100_CELL_HPP
