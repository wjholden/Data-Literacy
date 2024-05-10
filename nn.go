package main

import "fmt"

type nnet struct {
	w [][][]float32
	b [][]float32
	f func(float32) float32
}

func NewNNet(f func(float32) float32, layers ...int) *nnet {
	return nil
}

func (n *nnet) train(output []float32) {

}

func (n *nnet) test(input []float32) []float32 {
	return nil
}

func main() {
	fmt.Println("Hello, 世界")
}
