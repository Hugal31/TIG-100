//
// Created by laloge_h on 12/09/15.
//

#ifndef TIG_100_CELL_HPP
#define TIG_100_CELL_HPP

#define CELL_WIDTH  27
#define CELL_HEIGHT 17

/**
 * Basic cell
 */
class Cell
{
protected:

public:
    Cell();

    virtual void draw() const = 0;
};


#endif //TIG_100_CELL_HPP
