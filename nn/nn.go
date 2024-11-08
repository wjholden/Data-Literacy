package main

import (
	"fmt"
	"math/rand"
)

type nnet struct {
	w [][][]float64
	b [][]float64
	f func(float64) float64
}

func NewNNet(f func(float64) float64, layers ...int) *nnet {
	nlayers := len(layers)
	weights := make([][][]float64, nlayers-1)
	bias := make([][]float64, nlayers-1)
	for i := range nlayers - 1 {
		next_layer_size := layers[1+i]
		weights[i] = make([][]float64, next_layer_size)
		bias[i] = make([]float64, next_layer_size)
		for j := range next_layer_size {
			layer_size := layers[i]
			weights[i][j] = make([]float64, layer_size)
			for k := range layers[i] {
				weights[i][j][k] = 2*rand.Float64() - 1
			}
			bias[i][j] = 2*rand.Float64() - 1
		}
	}
	nn := nnet{w: weights, b: bias, f: f}
	return &nn
}

func (n *nnet) eval(x []float64) [][]float64 {
	y := make([][]float64, len(n.w))
	for i, layer := range n.w {
		layer_size := len(layer)
		y[i] = make([]float64, layer_size)
		for j, neuron := range layer {
			net := n.b[i][j]
			for k, weight := range neuron {
				net += x[k] * weight
			}
			y[i][j] = n.f(net)
		}
		x = y[i]
	}
	return y
}

func (n *nnet) train(x, y []float64) {

}

func (n *nnet) test(x []float64) []float64 {
	return n.eval(x)[len(n.w)-1]
}

func main() {
	relu := func(y float64) float64 {
		if y > 0 {
			return y
		} else {
			return 0
		}
	}
	nn := NewNNet(relu, 2, 2)
	fmt.Println(nn)
	nn.train([]float64{1.1, 2.2}, []float64{5.0})
	fmt.Println(nn.eval([]float64{1.1, 2.2}))
}
