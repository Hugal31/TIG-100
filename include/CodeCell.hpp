//
// Created by laloge_h on 12/09/15.
//

#ifndef TIG_100_CODECELL_HPP
#define TIG_100_CODECELL_HPP

#define NB_LINE 10

#include <array>
#include <string>
#include "Cell.hpp"

class CodeCell : public Cell
{
protected:
    std::array<std::string, NB_LINE> _code;

public:
    CodeCell(int x, int y);

    void draw();
};


#endif //TIG_100_CODECELL_HPP
