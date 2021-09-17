package main

import "fmt"

type Msg struct {
	in int
}

func producer(in []int) chan Msg {
	ch := make(chan Msg)
	go func() {
		for _, v := range in {
			ch <- Msg{in: v}
		}
		close(ch)
	}()
	return ch
}

func consumer(ch1, ch2 chan Msg) {
	for {
		select {
		case v1 := <-ch1:
			fmt.Println(v1)
		case v2 := <-ch2:
			fmt.Println(v2)
		}
	}
}

func mpsc() {
	ch1 := producer([]int{1, 2, 3})
	ch2 := producer([]int{4, 5, 6})

	consumer(ch1, ch2)
}
func main() {
	mpsc()
}
