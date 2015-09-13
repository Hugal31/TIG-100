//
// Created by laloge_h on 12/09/15.
//

#include <ncursesw/curses.h>
#include "termcap.hpp"
#include "CodeCell.hpp"

#define BORDER_H	"═"
#define BORDER_V	"║"
#define BORDER_NW	"╔"
#define BORDER_NE	"╗"
#define BORDER_SE	"╝"
#define BORDER_SW	"╚"

CodeCell::CodeCell() :
		_acc(0),
		_back(0),
		_status(STATUS_IDLE),
		_idle(0)
{
	_code[0] = "MOV ACC, DOWN";
	_code[1] = "HCF";
}

void CodeCell::draw_info(int start_x, int start_y)
{
	move(start_y + 2, start_x + CODE_NB_COLUMN + 4);
	printw("0"); // Nb to string ?

	move(start_y + 5, start_x + CODE_NB_COLUMN + 3);
	printw("<0>");
}

void CodeCell::draw_code(int start_x, int start_y)
{
	// Lignes de code
	for (unsigned int i(0); i < _code.size(); i++)
	{
		move(start_y + 1 + i, start_x + 1);
		printw(_code[i].c_str());
	}
};

void CodeCell::draw() // TODO Penser à utiliser un array de string
{
	int start_x, start_y;

	getyx(term, start_y, start_x);

	// Ligne du haut
	add_wch(WACS_D_ULCORNER); // TODO Utiliser les vrais caractères de box
	for (unsigned int x(1); x < CELL_WIDTH; x++)
		add_wch(WACS_D_HLINE);
	add_wch(WACS_D_URCORNER);

	// Ligne du bas
	move(start_y + CELL_HEIGHT - 1, start_x);
	add_wch(WACS_D_LLCORNER);
	for (unsigned int x(1); x < CELL_WIDTH; x++)
		add_wch(WACS_D_HLINE);
	add_wch(WACS_D_LRCORNER);

	// Lignes vericales

	for (unsigned int x(0); x <= CELL_WIDTH; x += CELL_WIDTH)
	{
		for (unsigned int y(1); y < CELL_HEIGHT - 1; y++)
		{
			move(start_y + y, start_x + x);
			add_wch(WACS_D_VLINE);
		}
	}

	// Block d'info
	for (unsigned int y(1); y < CELL_HEIGHT - 1; y++)
	{
		move(start_y + y, start_x + CODE_NB_COLUMN + 1);
		add_wch(WACS_D_VLINE);
	}

	for (unsigned int y(3); y < CELL_HEIGHT - 3; y += 3)
	{
		move (start_y + y, start_x + CODE_NB_COLUMN + 1);
		add_wch(WACS_D_LTEE);
		add_wch(WACS_D_VLINE);
		add_wch(WACS_D_VLINE);
		add_wch(WACS_D_VLINE);
		add_wch(WACS_D_VLINE);
		add_wch(WACS_D_VLINE);
		add_wch(WACS_D_VLINE);
		add_wch(WACS_D_RTEE);
	}

	move(start_y + 1, start_x + CODE_NB_COLUMN + 3);
	printw("ACC");

	move(start_y + 4, start_x + CODE_NB_COLUMN + 3);
	printw("BAK");

	move(start_y + 7, start_x + CODE_NB_COLUMN + 2);
	printw("STATUS");

	move(start_y + 10, start_x + CODE_NB_COLUMN + 3);
	printw("IDLE");

	draw_info(start_x, start_y);
	draw_code(start_x, start_y);
}
