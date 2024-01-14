package datainput

import (
	"encoding/csv"
	"fmt"
	"io"
	"os"
)

// ParseCSV validates and parses a CSV file with a single "text" column.
func ParseCSV(filePath string) ([]string, error) {
	// Open the CSV file
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	// Create a CSV reader
	reader := csv.NewReader(file)

	// Read and validate the header
	header, err := reader.Read()
	if err != nil {
		return nil, err
	}
	if len(header) != 1 || header[0] != "text" {
		return nil, fmt.Errorf("invalid CSV header. Expected a single column named 'text'")
	}

	var texts []string

	// Read and parse each record
	for {
		record, err := reader.Read()
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, err
		}

		if len(record) != 1 {
			return nil, fmt.Errorf("invalid number of columns in CSV. Expected only 'text'")
		}

		texts = append(texts, record[0])
	}

	return texts, nil
}
