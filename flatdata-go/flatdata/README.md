# flatdata-go
Common library for flatdata Go implementation

### Requirements
Library requires Go 1.9 and higher.

### Dependencies
Use [**dep**](https://github.com/golang/dep) to properly add `flatdata-go` to your project.    
Otherwise, you can always install dependencies manually:
```
go get -u golang.org/x/exp/mmap
go get -u github.com/stretchr/testify
```

### Tests and coverage
Run tests and show coverage info
```
go test -v -race ./... -coverprofile=coverage.out
```

Open coverage in browser
```
go tool cover -html=coverage.out
```

### Benchmarks
```
go test -bench=.
```
