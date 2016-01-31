//
// Created by laloge_h on 12/09/15.
//

#include "Ui/Ncurses.hpp"
#include "termcap.hpp"
#include "CodeCell.hpp"

using Ui::Ncurses;

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
  Ncurses::getInstance()->moveCursor(start_x + CODE_NB_COLUMN + 4, start_y + 2);
  printw("0"); // Nb to string ?

  Ncurses::getInstance()->moveCursor(start_y + 6, start_x + CODE_NB_COLUMN + 3);
  printw("<0>");
}

void CodeCell::draw_code(int start_x, int start_y) const
{
  // Lignes de code
  for (unsigned int i(0); i < _code.size(); i++)
    {
      Ncurses::getInstance()->moveCursor( start_x + 1, start_y + 1 + i);;
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
	Ncurses::getInstance()->moveCursor( start_x, start_y + CELL_HEIGHT - 1);;
	addch(ACS_LLCORNER);
	for (unsigned int x(1); x < CELL_WIDTH; x++)
		addch(ACS_HLINE);
	addch(ACS_LRCORNER);

	// Lignes vericales
	for (unsigned int x(0); x <= CELL_WIDTH; x += CELL_WIDTH)
	{
		for (unsigned int y(1); y < CELL_HEIGHT - 1; y++)
		{
			Ncurses::getInstance()->moveCursor( start_x + x, start_y + y);;
			addch(ACS_VLINE);
		}
	}

	// Block d'info
	for (unsigned int y(1); y < CELL_HEIGHT - 1; y++)
	{
		Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 1, start_y + y);;
		addch(ACS_VLINE);
	}

	Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 1, start_y);;
	addch(ACS_TTEE);

	for (unsigned int y(4); y < CELL_HEIGHT - 3; y += 4)
	{
		Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 1, start_y + y);;
		addch(ACS_LTEE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_HLINE);
		addch(ACS_RTEE);
	}

	Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 1, start_y + CELL_HEIGHT - 1);;
	addch(ACS_BTEE);

	Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 3, start_y + 1);;
	printw("ACC");

	Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 3, start_y + 5);;
	printw("BAK");

	Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 3, start_y + 9);;
	printw("LAST");

	Ncurses::getInstance()->moveCursor( start_x + CODE_NB_COLUMN + 3, start_y + 13);;
	printw("IDLE");

	draw_info(start_x, start_y);
	draw_code(start_x, start_y);
}
