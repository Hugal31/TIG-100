#include <array>
#include <iostream>
#include <ncursesw/curses.h>
#include <Cell.hpp>
#include <Grid.hpp>
#include "CodeCell.hpp"
#include "termcap.hpp"

using namespace std;

WINDOW *term;

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
        Grid grid;
		grid.draw();
        getch();
    }
    endwin();
    return 0;
}
