#

all: fmt vet test bench
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
		go test -v -race $$(go list ./... | grep -v /vendor/) -coverprofile=coverage.out
bench:
		echo "Run benchmarks..."
		go test -bench=.
