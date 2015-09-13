#include <array>
#include <iostream>
#include <ncursesw/curses.h>
#include <Cell.hpp>
#include "CodeCell.hpp"
#include "termcap.hpp"

using namespace std;

WINDOW *term;

static void draw_grid()
{
#define GRID_HEIGHT 3
#define GRID_WIDTH  4
    array<array<Cell*, GRID_WIDTH>, GRID_HEIGHT>    grid;

    for (unsigned int y(0); y < GRID_HEIGHT; y++)
    {
        for (unsigned int x(0); x < GRID_WIDTH; x++)
        {
            move(y * (CELL_HEIGHT + 1) + 2, x * (CELL_WIDTH + 5));
            grid[y][x] = new CodeCell;
            grid[y][x]->draw();
        }
    }
}

int main()
{
    int x, y;

    term = initscr();
    getmaxyx(term, y, x);
    if (false)// and x < MIN_WIDTH or y < MIN_HEIGHT)
    {
        cerr << "Please open a terminal with minimum "
        << MIN_WIDTH << " width and "
        << MIN_HEIGHT << " height" << endl;
    }
    else
    {
        noecho();
        printw("Press enter...");
        draw_grid();
        getch();
    }
    endwin();
    return 0;
}
