.PHONY: all run-ci clean lint test deps generate

all: clean lint generate test

run-ci: deps lint test

deps:
		@echo "Install dependencies..."
		dep ensure

clean:
		@echo "Cleanup..."
		find . -maxdepth 1 -type f -name "*_generated.go" -delete

lint:
		@echo "Run linters..."
		go fmt $$(go list ./... | grep -v /vendor/)
		go vet $$(go list ./... | grep -v /vendor/)
		golint $$(go list ./... | grep -v /vendor/) | grep -v _generated.go; test $$? -eq 1

generate:
		$$GOPATH/src/github.com/heremaps/flatdata/generator/app.py -v -g go \
            -s test_backward_compatibility.schema \
            -O test_backward_compatibility_generated.go

test:
		@echo "Run tests..."
		go test -v $$(go list ./... | grep -v /vendor/)
