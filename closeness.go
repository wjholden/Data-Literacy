package main

import "fmt"

var _ = map[string][]string{
	"start":     []string{"forest", "mountains", "sea", "city"},
	"forest":    []string{"start", "mountains", "desert", "cave"},
	"mountains": []string{"start", "forest", "glacier"},
	"desert":    []string{"forest"},
	"cave":      []string{"forest", "inferno"},
	"inferno":   []string{"cave"},
	"glacier":   []string{"mountains"},
	"sea":       []string{"start", "beach"},
	"beach":     []string{"sea", "city"},
	"city":      []string{"beach", "start", "castle"},
	"castle":    []string{"city", "treasure"},
	"treasure":  []string{"castle"}}

var G = map[string][]string{
	"alice": []string{"carol"},
	"bob":   []string{"carol"},
	"carol": []string{"eve", "dan"},
	"eve":   []string{"frank"},
	"dan":   []string{"frank"},
	"frank": []string{"gale"},
	"gale":  []string{}}

var _ = map[int][]int{
	1: []int{2},
	2: []int{1, 3},
	3: []int{2, 4},
	4: []int{3, 5},
	5: []int{4}}

// this procedure assumes the graph is directed
func bfs[T comparable](g map[T][]T, src, dst T) []T {
	parent := map[T]T{src: src}
	queue := []T{src}
	for len(queue) > 0 {
		position := queue[0]
		queue = queue[1:]
		if position == dst {
			break
		}
		for _, neighbor := range g[position] {
			if _, ok := parent[neighbor]; !ok {
				parent[neighbor] = position
				queue = append(queue, neighbor)
			}
		}
	}

	if _, ok := parent[dst]; !ok {
		return []T{} // dst not reachable from src
	}

	path := make([]T, 0)
	current := dst
	for current != src {
		path = append(path, current)
		current = parent[current]
	}
	return path
}

func main() {
	closeness()
}

func closeness() {
	for src, _ := range G {
		distances := 0.0
		for dst, _ := range G {
			path := bfs(G, src, dst)
			//fmt.Printf("%d -> %d path = %s\n", src, dst, path)
			distances += float64(len(path))
		}

		fmt.Printf("%d: %f\n", src, float64(len(G)-1)/distances)
		//fmt.Printf("%-10s: %f\n", src, 1.0/distances)
	}
}
