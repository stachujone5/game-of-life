package main

import (
	"errors"
	"fmt"
	"math/rand"
	"time"
)

type board struct {
	size  int
	cells []bool
	next  []bool
}

func New(size int) *board {
	randomized_cells := make([]bool, size*size)
	next := make([]bool, size*size)

	for i := 0; i < size*size; i++ {
		randomized_cells[i] = rand.Intn(2) == 1
	}

	return &board{
		size:  size,
		cells: randomized_cells,
		next:  next,
	}
}

func (b *board) get_num_of_alive_neighbours(x, y int) int {
	count := 0

	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			if dx == 0 && dy == 0 {
				continue
			}

			cell, err := b.get_cell(x+dx, y+dy)

			if err == nil && cell == true {
				count += 1
			}

		}
	}

	return count

}

func (b *board) get_row(index int) ([]bool, error) {
	if index < 0 || index >= len(b.cells)/b.size {
		return nil, errors.New("index out of bounds")
	}
	start := index * b.size
	end := start + b.size
	if end > len(b.cells) {
		return nil, errors.New("index out of bounds")
	}
	return b.cells[start:end], nil
}

func (b *board) get_cell(x, y int) (bool, error) {
	row, err := b.get_row(y)
	if err != nil {
		return false, err
	}

	if x < 0 || x >= b.size {
		return false, errors.New("invalid column index")
	}

	return row[x], nil
}

func (b *board) print() {
	for row_index := 0; row_index < b.size; row_index++ {
		row, err := b.get_row(row_index)

		if err != nil {
			panic(err)
		}

		row_str := ""

		for _, cell := range row {
			if cell {
				row_str += " # "
			} else {
				row_str += " . "
			}
		}

		fmt.Println(row_str)
	}
}

func (b *board) create_next_generation() {
	for x := 0; x < b.size; x++ {
		for y := 0; y < b.size; y++ {
			n := b.get_num_of_alive_neighbours(x, y)
			index := y*b.size + x

			var next bool

			switch b.cells[index] {
			case true:
				next = n == 2 || n == 3
			case false:
				next = n == 3
			}

			b.next[index] = next
		}
	}

	b.cells = b.next
	b.next = make([]bool, b.size*b.size)

}

func main() {
	board := New(45)

	for {
		board.print()
		board.create_next_generation()

		ten_millis := time.Duration(10) * time.Millisecond
		time.Sleep(ten_millis)
	}
}
