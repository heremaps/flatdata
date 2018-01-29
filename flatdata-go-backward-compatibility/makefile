all: clean fmt vet lint test
clean:
		echo "Cleanup..."
		find . -maxdepth 1 -type f -name "*_generated.go" -delete
deps:
		echo "Install dependencies..."
		go get -u github.com/golang/lint/golint
		go get -u github.com/golang/dep/cmd/dep
		dep ensure
fmt:
		echo "Run go fmt..."
		go fmt $$(go list ./... | grep -v /vendor/)
vet:
		echo "Run go vet..."
		go vet $$(go list ./... | grep -v /vendor/)
lint:
		echo "Run go lint..."
		golint -set_exit_status $$(go list ./... | grep -v /vendor/)
test:
		echo "Run tests..."
		$$GOPATH/src/github.com/heremaps/flatdata/generator/app.py -g go \
		-s test_backward_compatibility.schema \
		-O test_backward_compatibility_generated.go
		go test -v -race $$(go list ./... | grep -v /vendor/)