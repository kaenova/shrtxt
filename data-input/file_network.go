package datainput

import (
	"fmt"
	"io"
	"net/http"
	"os"
)

func DownloadAndSaveFile(url string) (string, error) {
	
	if os.IsNotExist(os.Mkdir("_temp", 0777)) {
		return "", fmt.Errorf("failed to create temporary directory")
	}

	// Create a temporary file
	tempFile, err := os.CreateTemp("_temp", "downloaded_file_")
	if err != nil {
		return "", err
	}
	defer tempFile.Close()

	// Make HTTP GET request to download the file
	response, err := http.Get(url)
	if err != nil {
		return "", err
	}
	defer response.Body.Close()

	// Check if the request was successful (status code 200)
	if response.StatusCode != http.StatusOK {
		return "", fmt.Errorf("failed to download file, status code: %d", response.StatusCode)
	}

	// Copy the file content to the temporary file
	_, err = io.Copy(tempFile, response.Body)
	if err != nil {
		return "", err
	}

	// Change file permissions
	err = os.Chmod(tempFile.Name(), 0777)
	if err != nil {
		return "", err
	}

	// Return the file path of the saved file
	return tempFile.Name(), nil
}

func DeleteFile(filePath string) error {
	err := os.Remove(filePath)
	if err != nil {
		return err
	}
	return nil
}