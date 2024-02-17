package main

import (
	"fmt"
	datainput "shrtxt-data-input"

	"github.com/nsqio/go-nsq"
)

var (
	NSQD_HOST               = datainput.EnvDefault("NSQD_HOST", "nsqd")
	NSQD_PORT               = datainput.EnvDefault("NSQD_PORT", "4150")
	NSQD_DATA_INPUT_TOPIC   = datainput.EnvDefault("NSQD_TOPIC", "data-input")

	DUMMY_PAYLOAD = "clrd02nme000508jv7lr76gq9"
)

func main() {
	// Connext to nsqd as a producer
	producer, err := nsq.NewProducer(fmt.Sprintf("%s:%s", NSQD_HOST, NSQD_PORT), nsq.NewConfig())
	if err != nil {
		panic(err)
	}

	// Defer the clean up of the producer
	defer producer.Stop()

	// Publish a message to the nsqd
	err = producer.Publish(NSQD_DATA_INPUT_TOPIC, []byte(DUMMY_PAYLOAD))
	if err != nil {
		panic(err)
	}

	fmt.Println("Message published to nsqd")
}