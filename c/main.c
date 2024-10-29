#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

struct Board {
  int size;
  bool *cells;
  bool *next;
};

struct Board createBoard(int size) {
  struct Board b;
  b.size = size;
  b.cells = malloc(size * size * sizeof(bool));
  b.next = malloc(size * size * sizeof(bool));

  for (int i = 0; i < size * size; i++) {
    b.cells[i] = rand() % 2;
    b.next[i] = false;
  }

  return b;
}

bool *get_row(struct Board *b, int index) {
  bool *row = malloc(b->size * sizeof(bool));
  for (int i = index * b->size; i < (index + 1) * b->size; i++) {
    row[i - index * b->size] = b->cells[i];
  }
  return row;
}

bool get_cell(struct Board *b, int x, int y) {
  bool *row = get_row(b, y);
  bool cell_value = row[x];
  free(row);
  return cell_value;
}

int get_num_of_alive_neighbours(struct Board *b, int x, int y) {
  int count = 0;

  for (int dx = -1; dx <= 1; dx++) {
    for (int dy = -1; dy <= 1; dy++) {
      if (dx == 0 && dy == 0) {
        continue;
      }
      int new_x = x + dx;
      int new_y = y + dy;
      if (new_x >= 0 && new_x < b->size && new_y >= 0 && new_y < b->size) {
        if (b->cells[new_x + b->size * new_y]) {
          count++;
        }
      }
    }
  }

  return count;
}

void create_next_generation(struct Board *b) {
  for (int x = 0; x < b->size; x++) {
    for (int y = 0; y < b->size; y++) {
      int n = get_num_of_alive_neighbours(b, x, y);
      int index = y * b->size + x;
      b->next[index] = false;

      if (b->cells[index]) {
        if (n == 2 || n == 3) {
          b->next[index] = true;
        }
      } else {
        if (n == 3) {
          b->next[index] = true;
        }
      }
    }
  }

  for (int i = 0; i < b->size * b->size; i++) {
    b->cells[i] = b->next[i];
  }
}

void print_board(struct Board *b) {
  printf("\e[1;1H\e[2J");
  for (int i = 0; i < b->size; i++) {
    for (int j = 0; j < b->size; j++) {
      if (b->cells[i * b->size + j]) {
        printf("#");
      } else {
        printf("*");
      }
    }
    printf("\n");
  }
}

int main() {
  srand(time(NULL));
  struct Board b = createBoard(50);

  while (true) {
    print_board(&b);
    create_next_generation(&b);
    usleep(100000);
  }

  free(b.cells);
  free(b.next);
  return 0;
}
