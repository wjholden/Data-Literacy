package main

import "fmt"

var g = map[string][]string{
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

func bfs(src, dst string) map[string]string {
	parent := map[string]string{src: src}
	queue := []string{src}
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
	return parent
}

func main() {
	tree := bfs("start", "treasure")
	fmt.Printf("Discovered treasure:")
	position := "treasure"
	for position != tree[position] {
		position = tree[position]
		fmt.Printf(" %s", position)
	}
	fmt.Println()

	closeness()
}

func closeness() {
	for src, _ := range g {
		distances := 0.0
		for dst, _ := range g {
			distances += float64(len(bfs(src, dst)))
		}

		fmt.Printf("%-10s: %f\n", src, float64(len(g)-1)/distances)
	}
}
