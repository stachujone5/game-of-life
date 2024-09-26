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

func newBoard(size int) *board {
	randomizedCells := make([]bool, size*size)
	next := make([]bool, size*size)

	for i := 0; i < size*size; i++ {
		randomizedCells[i] = rand.Intn(2) == 1
	}

	return &board{
		size:  size,
		cells: randomizedCells,
		next:  next,
	}
}

func (b *board) getNumOfAliveNeighbours(x, y int) int {
	count := 0

	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			if dx == 0 && dy == 0 {
				continue
			}

			cell, err := b.getCell(x+dx, y+dy)

			if err == nil && cell == true {
				count += 1
			}

		}
	}

	return count

}

func (b *board) getRow(index int) ([]bool, error) {
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

func (b *board) getCell(x, y int) (bool, error) {
	row, err := b.getRow(y)
	if err != nil {
		return false, err
	}

	if x < 0 || x >= b.size {
		return false, errors.New("invalid column index")
	}

	return row[x], nil
}

func (b *board) print() error {
	for rowIndex := 0; rowIndex < b.size; rowIndex++ {
		row, err := b.getRow(rowIndex)

		if err != nil {
			return err
		}

		rowStr := ""

		for _, cell := range row {
			if cell {
				rowStr += " # "
			} else {
				rowStr += " . "
			}
		}

		fmt.Println(rowStr)
	}

	return nil
}

func (b *board) createNextGeneration() {
	for x := 0; x < b.size; x++ {
		for y := 0; y < b.size; y++ {
			n := b.getNumOfAliveNeighbours(x, y)
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
	board := newBoard(45)

	for {
		err := board.print()

		if err != nil {
			panic(err)
		}

		board.createNextGeneration()

		tenMillis := time.Duration(10) * time.Millisecond
		time.Sleep(tenMillis)
	}
}
