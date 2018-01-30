GODIR=$$GOPATH/src/github.com/heremaps/flatdata/flatdata-go

all: check test bench

run-ci: deps all

setup-ci:
		@echo "Setup for CI..."
		mkdir -p $(GODIR)
		cp -R . $(GODIR)

deps:
		@echo "Install dependencies..."
		dep ensure

check:
		@echo "Run checks..."
		go fmt $$(go list ./... | grep -v /vendor/)
		go vet $$(go list ./... | grep -v /vendor/)
		golint -set_exit_status $$(go list ./... | grep -v /vendor/)

test:
		@echo "Run tests..."
		go test -v -race $$(go list ./... | grep -v /vendor/) -coverprofile=coverage.out

bench:
		@echo "Run benchmarks..."
		go test -bench=.
