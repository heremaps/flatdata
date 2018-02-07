.PHONY: all run-ci clean check test

all: clean check test

run-ci: deps check test

deps:
		@echo "Install dependencies..."
		dep ensure

clean:
		@echo "Cleanup..."
		find . -maxdepth 1 -type f -name "*_generated.go" -delete

check:
		@echo "Run checks..."
		go fmt $$(go list ./... | grep -v /vendor/)
		go vet $$(go list ./... | grep -v /vendor/)
		golint -set_exit_status $$(go list ./... | grep -v /vendor/)

test:
		@echo "Run tests..."
		go test -v $$(go list ./... | grep -v /vendor/)
