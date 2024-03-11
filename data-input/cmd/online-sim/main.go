package main

import (
	"fmt"
	datainput "shrtxt-data-input"

	"github.com/nsqio/go-nsq"
)

var (
	NSQD_CONSUMER_HOST      = datainput.EnvDefault("NSQD_CONSUMER_HOST", "nsqlookupd")
	NSQD_CONSUMER_PORT      = datainput.EnvDefault("NSQD_CONSUMER_PORT", "4161")
	NSQD_PRODUCER_HOST      = datainput.EnvDefault("NSQD_PRODUCER_HOST", "nsqd")
	NSQD_PRODUCER_PORT      = datainput.EnvDefault("NSQD_PRODUCER_PORT", "4150")
	NSQD_DATA_INPUT_TOPIC = datainput.EnvDefault("NSQD_TOPIC", "data-input")

	DUMMY_PAYLOAD = "clsr02t9i000008jq38rg9he1"
)

func main() {
	// Connext to nsqd as a producer
	producer, err := nsq.NewProducer(fmt.Sprintf("%s:%s", NSQD_PRODUCER_HOST, NSQD_PRODUCER_PORT), nsq.NewConfig())
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
