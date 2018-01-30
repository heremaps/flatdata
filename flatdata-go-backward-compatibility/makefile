GODIR=$$GOPATH/src/github.com/heremaps/flatdata/flatdata-go-backward-compatibility

all: clean check test

run-ci: deps all

setup-ci:
		@echo "Setup for CI..."
		mkdir -p $(GODIR)
		cp -R . $(GODIR)

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
		$$GOPATH/src/github.com/heremaps/flatdata/generator/app.py -g go \
		-s test_backward_compatibility.schema \
		-O test_backward_compatibility_generated.go
		go test -v -race $$(go list ./... | grep -v /vendor/)
