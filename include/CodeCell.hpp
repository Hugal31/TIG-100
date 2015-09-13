//
// Created by laloge_h on 12/09/15.
//

#ifndef TIG_100_CODECELL_HPP
#define TIG_100_CODECELL_HPP

#define CODE_NB_ROW      15
#define CODE_NB_COLUMN   19

#include <array>
#include <string>
#include "Cell.hpp"

enum Status
{
	STATUS_IDLE,
	STATUS_READ,
	STATUS_WRITE
};

class CodeCell : public Cell
{
protected:
    short int   _acc;
    short int   _back;
	Status		_status;
    short int   _idle;
    std::array<std::string, CODE_NB_ROW> _code;

	void draw_info(int start_x, int start_y);
    void draw_code(int start_x, int start_y);

public:
    CodeCell();

    void draw();
};


#endif //TIG_100_CODECELL_HPP
