.PHONY: all run-ci lint test bench deps

all: lint test bench

run-ci: deps all

deps:
		@echo "Install dependencies..."
		dep ensure

lint:
		@echo "Run linters..."
		go fmt $$(go list ./... | grep -v /vendor/)
		go vet $$(go list ./... | grep -v /vendor/)
		golint -set_exit_status $$(go list ./... | grep -v /vendor/)

test:
		@echo "Run tests..."
		go test -v $$(go list ./... | grep -v /vendor/)

bench:
		@echo "Run benchmarks..."
		go test -bench=.
