package main

import (
	"fmt"
)

func main() {
	mpsc()
	select {}
}

type Msg struct {
	in int
}

// 生产者
func producer(sendChan chan Msg) {
	for i := 0; i < 10; i++ {
		sendChan <- Msg{in: i}
	}
}

// 消费者
func consumer(sendChan chan Msg) {
	for v := range sendChan {
		process(v)
	}
}

// 消息处理函数
func process(msg Msg) {
	fmt.Println(msg)
}

func mpsc() {

	sendChan := make(chan Msg, 10)

	for p := 0; p < 3; p++ {
		go producer(sendChan)
	}

	go consumer(sendChan)
}
