package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

type helloHandler struct{}

func (h helloHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "hello, you've hit %s\n", r.URL.Path)
}

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
		log.Printf("Defaulting to port %s", port)
	}

	log.Printf("Starting server on port %s", port)
	err := http.ListenAndServe(":"+port, helloHandler{})
 	log.Fatal(err)
}
