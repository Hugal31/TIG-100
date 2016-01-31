//
// Ncurses.cpp for cpp_gkrellm in /home/laloge_h/projets/Piscine_CPP/cpp_gkrellm
//
// Made by Hugo Laloge
// Login   <laloge_h@epitech.net>
//
// Started on  Sat Jan 23 12:52:59 2016 Hugo Laloge
// Last update Sat Jan 23 13:22:51 2016 Hugo Laloge
//

#include <ncurses.h>
#include <iostream>
#include "Ui/Ncurses.hpp"

Ui::Ncurses::Ncurses()
{
  _term = initscr();
  _has_colors = has_colors();
  noecho();
  timeout(1000);
  scrollok(_term, true);
  if (_has_colors)
  {
    start_color();
    use_default_colors();
    init_pair(DEFAULT_COLOR, -1 , -1);
    init_pair(WHITE_ON_GREEN, COLOR_WHITE, COLOR_GREEN);
    init_pair(WHITE_ON_RED, COLOR_WHITE, COLOR_RED);
    init_pair(GREEN_ON_BLACK, COLOR_GREEN, -1);
    init_pair(RED_ON_BLACK, COLOR_RED, -1);
    init_pair(BLACK_ON_WHITE, COLOR_WHITE, COLOR_CYAN);
  }
  moveCursor(0, 0);
}

Ui::Ncurses::~Ncurses()
{
  endwin();
}

Ui::Ncurses *Ui::Ncurses::getInstance()
{
  static Ncurses instance;
  return &instance;
}

void Ui::Ncurses::refresh()
{
  ::refresh();
}

void Ui::Ncurses::startColor(Ui::Ncurses::ColorCombination combination)
{
  attron(COLOR_PAIR(combination));
}

void Ui::Ncurses::startBold()
{
  attron(A_BOLD);
}

void Ui::Ncurses::reset()
{
  attroff(A_BOLD);
  attron(COLOR_PAIR(DEFAULT_COLOR));
}

void Ui::Ncurses::write(const std::string &string)
{
  printw(string.c_str());
}

void Ui::Ncurses::writeInColor(const std::string &string, ColorCombination combination)
{
  startColor(combination);
  printw(string.c_str());
  startColor(DEFAULT_COLOR);
}

int Ui::Ncurses::getMaxX()
{
  return getmaxx(_term);
}

int Ui::Ncurses::getMaxY()
{
  return getmaxy(_term);
}

int Ui::Ncurses::getCursorX()
{
  return getcurx(_term);
}

int Ui::Ncurses::getCursorY()
{
  return getcury(_term);
}

void Ui::Ncurses::setCursorX(int x)
{
  moveCursor(x, getCursorY());
}

void Ui::Ncurses::setCursorY(int y)
{
  moveCursor(getCursorX(), y);
}

void Ui::Ncurses::moveCursor(int x, int y)
{
  wmove(_term, y, x);
}

void Ui::Ncurses::carriageReturn()
{
  moveCursor(0, getCursorY() + 1);
}

Ui::Ncurses &Ui::Ncurses::operator<<(const std::string &string)
{
  write(string);
  return *this;
}

Ui::Ncurses &Ui::Ncurses::operator<<(const char *string)
{
  printw(string);
  return *this;
}

Ui::Ncurses &Ui::Ncurses::operator<<(Ui::Ncurses::ColorCombination combination)
{
  startColor(combination);
  return *this;
}

void Ui::Ncurses::clear()
{
  erase();
}
