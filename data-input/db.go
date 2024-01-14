package datainput

import (
	"database/sql"
)

func ChangeProjectStatus(tx *sql.Tx, projectId string, status string) error {
	sqlStatement := `
		UPDATE "Project"
		SET "status" = $1
		WHERE "id" = $2;
	`
	_, err := tx.Exec(sqlStatement, status, projectId)
	if err != nil {
		return err
	}
	return nil
}

// Input Data Batch with Commit and Rollback with Go Concurrency
func InputTextDataBatch(tx *sql.Tx, projectId string, data []string) ([]int, error) {
	// Insert data into the "example" table
	sqlStatement := `
		INSERT INTO "Text" ("value", "projectId") 
		VALUES ($1, $2)
		RETURNING id;
	`

	// Execute the SQL statement and retrieve the inserted ID
	var insertedID int
	var insertedIDs []int = make([]int, 0)
	var err error

	for _, d := range data {
		err = tx.QueryRow(sqlStatement, d, projectId).Scan(&insertedID)
		if err != nil {
			tx.Rollback()
			return insertedIDs, err
		}
		insertedIDs = append(insertedIDs, insertedID)
	}
	if err != nil {
		return insertedIDs, err
	}
	return insertedIDs, nil
}