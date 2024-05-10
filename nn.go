package main

import (
	"fmt"
	"math/rand"
)

type nnet struct {
	w [][][]float32
	b [][]float32
	f func(float32) float32
}

func NewNNet(f func(float32) float32, layers ...int) *nnet {
	nlayers := len(layers)
	weights := make([][][]float32, nlayers-1)
	bias := make([][]float32, nlayers-1)
	for i := range nlayers - 1 {
		next_layer_size := layers[1+i]
		weights[i] = make([][]float32, next_layer_size)
		bias[i] = make([]float32, next_layer_size)
		for j := range next_layer_size {
			layer_size := layers[i]
			weights[i][j] = make([]float32, layer_size)
			for k := range layers[i] {
				weights[i][j][k] = 2*rand.Float32() - 1
			}
			bias[i][j] = 2*rand.Float32() - 1
		}
	}
	nn := nnet{w: weights, b: bias, f: f}
	return &nn
}

func (n *nnet) train(x, y []float32) {

}

func (n *nnet) test(x []float32) []float32 {
	return nil
}

func main() {
	relu := func(y float32) float32 {
		if y > 0 {
			return y
		} else {
			return 0
		}
	}
	nn := NewNNet(relu, 2, 3, 1)
	fmt.Println(nn)
}
