package main

import (
	"database/sql"
	"fmt"
	"log"
	datainput "shrtxt-data-input"

	_ "github.com/lib/pq"
)

const (
	host     = "db"
	port     = 5432
	user     = "postgres"
	password = "changeme"
	dbname   = "main"

	fileUrl = "https://gist.githubusercontent.com/kaenova/4782cbffbfd362413e2028d99612976a/raw/520d58f05aaf7dd7a5b288d07476a6be9a42f1b9/mock.csv"
)

func main() {
	// Construct the connection string
	connStr := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable", host, port, user, password, dbname)

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

	fmt.Println("Successfully connected!")

	filePath, err := datainput.DownloadAndSaveFile(fileUrl)
	if err != nil {
		log.Fatal(err)
	}
	defer datainput.DeleteFile(filePath)

	data, err := datainput.ParseCSV(filePath)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(len(data))

	tx, err := db.Begin()
	if err != nil {
		log.Fatal(err)
	}

	err = datainput.ChangeProjectStatus(tx, "clrd02nme000508jv7lr76gq9", "In Progress: Data Input")
	if err != nil {
		tx.Rollback()
		log.Fatal(err)
	}

	ids, err := datainput.InputTextDataBatch(tx, "clrd02nme000508jv7lr76gq9", data)
	if err != nil {
		tx.Rollback()
		log.Fatal(err)
	}

	err = datainput.ChangeProjectStatus(tx, "clrd02nme000508jv7lr76gq9", "In Queue")
	if err != nil {
		tx.Rollback()
		log.Fatal(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		log.Fatal(err)
	}

	fmt.Println("Successfully inserted data!", len(ids))
}
