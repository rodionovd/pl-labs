package main

import "fmt"

type Token struct {
	Recipient int
	Message string
}

// The main routine is going to be blocked until all workers are done. We'll send
// a message down this channel when it's time to proceed
var done = make(chan bool)

// Each worker reads a token from the input. If the token's receipient matches
// this worker's ID then a message is printed; otherwise this token is forwarded
// to the next worker in a queue
func worker(id int, input chan Token, output chan Token) {
	if t := <-input; t.Recipient == id {
		fmt.Printf("Worker %d got message: %s\n", id, t.Message)
		done <- true
	} else {
		fmt.Printf("Worker %d forwards message for %d to the next worker\n",
		           id, t.Recipient)
		output <- t
	}
}

const WORKERS = 10
const RECEIVER = 9

func main() {
	if RECEIVER >= WORKERS || RECEIVER < 0 {
		fmt.Println("Invalid RECEIVER value")
		return
	}

	input := make(chan Token, 1)
	input <- Token{RECEIVER, "hi there"}

	for i := 0; i < WORKERS; i++ {
		// Each worker's input is a previous worker's output, so n-th
		// worker can pass messages to (n+1)-th one
		output := make(chan Token)
		go worker(i, input, output)
		input = output
	}

	// Wait for workers
	<-done
}
