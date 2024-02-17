package datainput

import (
	"database/sql"
)

type Project struct {
	ID      string
	Status  string
	FileUrl string
}

func GetProjectByID(db *sql.DB, projectId string) (Project, error) {
	var project Project
	sqlStatement := `
		SELECT id, status, "fileUrl" FROM "Project" WHERE id = $1;
	`
	row := db.QueryRow(sqlStatement, projectId)
	err := row.Scan(&project.ID, &project.Status, &project.FileUrl)
	if err != nil {
		return project, err
	}
	return project, nil
}

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
	// Delete all text with the same project id
	sqlStatement := `
		DELETE FROM "Text" WHERE "projectId" = $1;
	`
	_, err := tx.Exec(sqlStatement, projectId)
	if err != nil {
		return nil, err
	}
	
	// Insert data into the "example" table
	sqlStatement = `
		INSERT INTO "Text" ("value", "projectId") 
		VALUES ($1, $2)
		RETURNING id;
	`

	// Execute the SQL statement and retrieve the inserted ID
	var insertedID int
	var insertedIDs []int = make([]int, 0)

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
