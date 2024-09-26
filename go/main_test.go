package main

import (
	"reflect"
	"testing"
)

func TestBoardNew(t *testing.T) {
	size := 5
	board := new(size)

	if board.size != size {
		t.Fatalf("New(%d).size = %d; want %d", size, board.size, size)
	}

}

func TestGetCell(t *testing.T) {
	size := 3
	board := board{
		size:  size,
		cells: []bool{true, false, true, false, true, false, true, false, true},
		next:  []bool{false, false, false, false, false, false, false, false, false},
	}

	tests := []struct {
		x, y      int
		expected  bool
		shouldErr bool
	}{
		{0, 0, true, false},
		{1, 1, true, false},
		{2, 2, true, false},
		{2, 1, false, false},
		{3, 3, false, true},
	}

	for _, test := range tests {
		result, err := board.getCell(test.x, test.y)

		if test.shouldErr {
			if err == nil {
				t.Errorf("getCell(%d, %d) expected an error; got none", test.x, test.y)
			}
		} else {
			if err != nil {
				t.Errorf("getCell(%d, %d) returned an error: %v; want no error", test.x, test.y, err)
			} else if result != test.expected {
				t.Errorf("getCell(%d, %d) = %v; want %v", test.x, test.y, result, test.expected)
			}
		}
	}
}

func TestGetRow(t *testing.T) {
	size := 3
	b := board{
		size:  size,
		cells: []bool{true, false, true, false, true, false, true, false, true},
	}

	tests := []struct {
		row       int
		expected  []bool
		shouldErr bool
	}{
		{0, []bool{true, false, true}, false},
		{1, []bool{false, true, false}, false},
		{2, []bool{true, false, true}, false},
		{3, nil, true},
	}

	for _, test := range tests {
		result, err := b.getRow(test.row)

		if test.shouldErr {
			if err == nil {
				t.Errorf("getRow(%d) expected an error; got none", test.row)
			}
		} else {
			if err != nil {
				t.Errorf("getRow(%d) returned an error: %v; want no error", test.row, err)
			} else if !reflect.DeepEqual(result, test.expected) {
				t.Errorf("getRow(%d) = %v; want %v", test.row, result, test.expected)
			}
		}
	}
}

func TestGetNumOfAliveNeighbours(t *testing.T) {
	size := 3
	board := board{
		size:  size,
		cells: []bool{true, false, true, false, true, false, true, false, true},
		next:  []bool{false, false, false, false, false, false, false, false, false},
	}

	tests := []struct {
		x, y      int
		expected  int
		shouldErr bool
	}{
		{0, 0, 1, false},
		{1, 1, 4, false},
		{2, 2, 1, false},
	}

	for _, test := range tests {
		result := board.getNumOfAliveNeighbours(test.x, test.y)

		if test.shouldErr {
			if result != 0 {
				t.Errorf("getNumOfAliveNeighbours(%d, %d) returned %d; want 0", test.x, test.y, result)
			}
		} else {
			if result != test.expected {
				t.Errorf("getNumOfAliveNeighbours(%d, %d) = %d; want %d", test.x, test.y, result, test.expected)
			}
		}
	}
}

func TestCreateNextGeneration(t *testing.T) {
	size := 3
	b := board{
		size:  size,
		cells: []bool{false, true, false, false, true, false, false, true, false},
		next:  make([]bool, size*size),
	}

	b.createNextGeneration()

	expectedNextGen := []bool{false, false, false, true, true, true, false, false, false}

	for i := range expectedNextGen {
		if b.cells[i] != expectedNextGen[i] {
			t.Errorf("createNextGeneration() = %v; want %v", b.cells, expectedNextGen)
			break
		}
	}
}
