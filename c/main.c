#include <stdbool.h>
#include <stdlib.h>

struct Board {
  int size;
  bool cells[45];
  bool next[45];
};

struct Board createBoard(int size) {
  struct Board b;
  b.size = size;

  for (int i = 0; i < size * size; i++) {
    b.cells[i] = rand() % 2;
  }

  for (int i = 0; i < size * size; i++) {
    b.next[i] = false;
  }

  return b;
}

int main() { return 0; }
