package main

import (
	"database/sql"
	"fmt"
	"log"
	datainput "shrtxt-data-input"
	"strconv"

	"github.com/nsqio/go-nsq"

	_ "github.com/lib/pq"
)

var (
	MAXIMUM_ROWS = datainput.EnvDefault("MAXIMUM_ROWS", "100000")

	DB_HOST     = datainput.EnvDefault("DB_HOST", "db")
	DB_PORT     = datainput.EnvDefault("DB_PORT", "5432")
	DB_USER     = datainput.EnvDefault("DB_USER", "postgres")
	DB_PASSWORD = datainput.EnvDefault("DB_PASSWORD", "changeme")
	DB_NAME     = datainput.EnvDefault("DB_NAME", "main")

	NSQD_HOST               = datainput.EnvDefault("NSQD_HOST", "nsqd")
	NSQD_PORT               = datainput.EnvDefault("NSQD_PORT", "4150")
	NSQD_DATA_INPUT_TOPIC   = datainput.EnvDefault("NSQD_DATA_INPUT_TOPIC", "data-input")
	NSQD_DATA_INPUT_CHANNEL = datainput.EnvDefault("NSQD_DATA_INPUT_CHANNEL", "data-input-worker")
	NSQD_EMBED_TEXT_TOPIC   = datainput.EnvDefault("NSQD_EMBED_TEXT_TOPIC", "embed-text")
)

func main() {
	// Handle Environment Variables Parsing
	maxRows, err := strconv.Atoi(MAXIMUM_ROWS)
	if err != nil {
		log.Fatal(err)
	}

	// Construct the connection string
	connStr := fmt.Sprintf("host=%s port=%s user=%s password=%s dbname=%s sslmode=disable", DB_HOST, DB_PORT, DB_USER, DB_PASSWORD, DB_NAME)

	// Open a connection to the database
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// Attempt to ping the database to check the connection
	err = db.Ping()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Database Successfully connected!")

	// Conncet to nsqd as a consumer
	consumer, err := nsq.NewConsumer(NSQD_DATA_INPUT_TOPIC, NSQD_DATA_INPUT_CHANNEL, nsq.NewConfig())
	if err != nil {
		log.Fatal(err)
	}

	// Connect to nsqd as a producer
	producer, err := nsq.NewProducer(fmt.Sprintf("%s:%s", NSQD_HOST, NSQD_PORT), nsq.NewConfig())
	if err != nil {
		log.Fatal(err)
	}

	// Defer the clean up of the producer
	defer producer.Stop()

	// Defer the clean up of the consumer
	defer consumer.Stop()

	// Set the handler for the consumer with the message as a project id
	consumer.AddHandler(nsq.HandlerFunc(func(message *nsq.Message) error {
		fmt.Println("Received message Project ID:", string(message.Body))

		// Get the project by the project id
		project, err := datainput.GetProjectByID(db, string(message.Body))
		if err != nil {
			fmt.Println(err)
			return err
		}

		if (project.Status != "[0/4 Steps] In Queue") {
			fmt.Printf("Project %s status is not '[0/4 Steps] In Queue'\n", string(message.Body))
			return nil
		}

		// Start a transaction
		tx, err := db.Begin()
		if err != nil {
			fmt.Println(err)
			return err
		}
		defer tx.Rollback()

		// Update the project status to "In Progress: Data Input"
		err = datainput.ChangeProjectStatus(tx, project.ID, "[0/4 Steps] Processing")
		if err != nil {
			fmt.Println(err)
			return err
		}

		// Download the file and save it to a temporary directory
		fmt.Println("Downloading and saving the file")
		filePath, err := datainput.DownloadAndSaveFile(project.FileUrl)
		if err != nil {
			fmt.Println(err)
			return err	
		}

		// Defer the deletion of the file
		defer datainput.DeleteFile(filePath)

		// Parse the CSV file
		// If fail then it's an invalid CSV file format
		fmt.Println("Parsing CSV file")
		data, err := datainput.ParseCSV(filePath)
		if err != nil {
			fmt.Println(err)

			// Update the project status to "Failed"
			err = datainput.ChangeProjectStatus(tx, project.ID, "Failed: Invalid CSV File Format")
			if err != nil {
				fmt.Println(err)
				return err
			}
			
			// Commit the transaction
			err = tx.Commit()
			if err != nil {
				fmt.Println(err)
				return err
			}

			return nil
		}

		// Handle Big Data
		if (len(data) > maxRows) {
			fmt.Println("Data exceeds maximum rows")
			err = datainput.ChangeProjectStatus(tx, project.ID, "Failed: Data Exceeds Maximum Rows")
			if err != nil {
				fmt.Println(err)
				return err
			}

			tx.Commit()
			return nil
		}


		// Insert the data into the database
		fmt.Println("Inserting data into the database")
		_, err = datainput.InputTextDataBatch(tx, project.ID, data)
		if err != nil {
			fmt.Println(err)
			return err
		}


		// Queue the project for the next step
		fmt.Println("Queueing the project for the next step")
		err = producer.Publish(NSQD_EMBED_TEXT_TOPIC, []byte(project.ID))
		if err != nil {
			fmt.Println(err)
			return err
		}

		// Update the project status to "In Queue"
		fmt.Println("Updating the project status to 'In Queue'")
		err = datainput.ChangeProjectStatus(tx, project.ID, "[1/4 Steps] In Queue")
		if err != nil {
			fmt.Println(err)
			return err
		}

		// Commit the transaction
		fmt.Println("Committing the transaction")
		err = tx.Commit()
		if err != nil {
			fmt.Println(err)
			return err
		}

		return nil
	},
	))

	// Connect the consumer to nsqd
	err = consumer.ConnectToNSQD(fmt.Sprintf("%s:%s", NSQD_HOST, NSQD_PORT))
	if err != nil {
		log.Fatal(err)
	}

	// Block the main thread
	select {}
}
