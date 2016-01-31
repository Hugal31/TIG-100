//
// Created by laloge_h on 12/09/15.
//

#include <ncurses.h>
#include "termcap.hpp"
#include "CodeCell.hpp"

CodeCell::CodeCell()
  : _acc(0)
  , _back(0)
  , _status(STATUS_IDLE)
  , _idle(0)
{
	_code[0] = "MOV ACC, DOWN";
	_code[1] = "HCF";
}

void CodeCell::draw_info(int start_x, int start_y) const
{
	move(start_y + 2, start_x + CODE_NB_COLUMN + 4);
	printw("0"); // Nb to string ?

	move(start_y + 6, start_x + CODE_NB_COLUMN + 3);
	printw("<0>");
}

void CodeCell::draw_code(int start_x, int start_y) const
{
  // Lignes de code
  for (unsigned int i(0); i < _code.size(); i++)
    {
      move(start_y + 1 + i, start_x + 1);
      printw(_code[i].c_str());
    }
}

void CodeCell::draw() const // TODO Penser à utiliser un array de string
{
	int start_x, start_y;

	getyx(term, start_y, start_x);

	// Ligne du haut
	addch(ACS_ULCORNER); // TODO Utiliser les vrais caractères de box
	for (unsigned int x(1); x < CELL_WIDTH; x++)
		addch(ACS_HLINE);
	addch(ACS_URCORNER);

	// Ligne du bas
	move(start_y + CELL_HEIGHT - 1, start_x);
	addch(ACS_LLCORNER);
	for (unsigned int x(1); x < CELL_WIDTH; x++)
		addch(ACS_HLINE);
	addch(ACS_LRCORNER);

	// Lignes vericales
	for (unsigned int x(0); x <= CELL_WIDTH; x += CELL_WIDTH)
	{
		for (unsigned int y(1); y < CELL_HEIGHT - 1; y++)
		{
			move(start_y + y, start_x + x);
			addch(ACS_VLINE);
		}
	}

	// Block d'info
	for (unsigned int y(1); y < CELL_HEIGHT - 1; y++)
	{
		move(start_y + y, start_x + CODE_NB_COLUMN + 1);
		addch(ACS_VLINE);
	}

	move(start_y, start_x + CODE_NB_COLUMN + 1);
	addch(ACS_TTEE);

	for (unsigned int y(4); y < CELL_HEIGHT - 3; y += 4)
	{
		move(start_y + y, start_x + CODE_NB_COLUMN + 1);
		addch(ACS_LTEE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_RTEE);
	}

	move(start_y + CELL_HEIGHT - 1, start_x + CODE_NB_COLUMN + 1);
	addch(ACS_BTEE);

	move(start_y + 1, start_x + CODE_NB_COLUMN + 3);
	printw("ACC");

	move(start_y + 5, start_x + CODE_NB_COLUMN + 3);
	printw("BAK");

	move(start_y + 9, start_x + CODE_NB_COLUMN + 3);
	printw("LAST");

	move(start_y + 13, start_x + CODE_NB_COLUMN + 3);
	printw("IDLE");

	draw_info(start_x, start_y);
	draw_code(start_x, start_y);
}
