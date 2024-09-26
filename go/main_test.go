package main

import "testing"

func TestGreet(t *testing.T) {
	result := greet("Alice")
	expected := "Hello, Alice!"

	if result != expected {
		t.Errorf("greet(\"Alice\") = %q; want %q", result, expected)
	}
}
