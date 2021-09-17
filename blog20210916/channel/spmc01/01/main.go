package main

import "fmt"

type Msg struct {
	in int
}

func producer(ch chan Msg) {
	in := []int{1, 2, 3, 4, 5, 6}
	for _, v := range in {
		ch <- Msg{in: v}
	}
	close(ch)

}

func consumer(ch chan Msg) {
	for v := range ch {
		fmt.Println(v)
	}
}

func spmc() {

	ch := make(chan Msg)
	go producer(ch)
	go consumer(ch)
	go consumer(ch)
	go consumer(ch)

}

func main() {
	spmc()
	select {}
}
