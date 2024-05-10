package main

import "fmt"

type nnet struct {
	w [][][]float32
	b [][]float32
}

func (n *nnet) evaluate(input []float32) float32 {
	return 0.0
}

func (n *nnet) backpropagate(output []float32) {

}

func main() {
	fmt.Println("Hello, 世界")
}
