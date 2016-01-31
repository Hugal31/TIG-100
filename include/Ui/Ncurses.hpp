//
// Ncurses.hpp for cpp_gkrellm in /home/laloge_h/projets/Piscine_CPP/cpp_gkrellm
//
// Made by Hugo Laloge
// Login   <laloge_h@epitech.net>
//
// Started on  Sat Jan 23 12:52:58 2016 Hugo Laloge
// Last update Sat Jan 23 12:52:58 2016 Hugo Laloge
//

#ifndef UI_NCURSES_HPP
#define UI_NCURSES_HPP

#include <string>
#include <ncurses.h>

namespace Ui
{
  /**
   * Abstraction for ncurses call
   */
  class Ncurses
  {
  public:
    enum ColorCombination
    {
      DEFAULT_COLOR = 1,
      WHITE_ON_GREEN,
      WHITE_ON_RED,
      GREEN_ON_BLACK,
      RED_ON_BLACK,
      BLACK_ON_WHITE
    };

  public:
    static Ncurses *getInstance();

    // Write functions
    void startColor(ColorCombination);
    void startBold();
    void reset();
    void write(const std::string &);
    void writeInColor(const std::string &, ColorCombination);
    void refresh();
    void clear();

    // Term info functions
    int getMaxX();
    int getMaxY();

    // Cursor position functions
    int getCursorX();
    int getCursorY();
    void setCursorX(int x);
    void setCursorY(int y);
    void moveCursor(int x, int y);
    void carriageReturn();

    Ncurses &operator<<(const std::string &);
    Ncurses &operator<<(const char *);
    Ncurses &operator<<(ColorCombination);

  private:
    Ncurses();
    ~Ncurses();
    Ncurses(const Ncurses &) = delete;
    Ncurses &operator=(const Ncurses &) = delete;

  private:
    WINDOW *_term;
    bool _has_colors;
  };
}

#endif //NCURSES_HPP
