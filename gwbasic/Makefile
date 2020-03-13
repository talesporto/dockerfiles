.PHONY: all run-perf perf docker-build-standalone docker-build-httpd clean

PERF_EXE = ./perf/target/release/perf.exe
GWBASIC_EXE = ./bin/GWBASIC.EXE
# TODO fix hardcoded unix PWD
PWD_UNIX = /c/Users/ngeor/Projects/github/dockerfiles/gwbasic

all: perf docker-build-standalone docker-build-httpd

run-perf: perf
	GWBASIC=$(GWBASIC_EXE) $(PERF_EXE)

perf: $(PERF_EXE)

$(PERF_EXE): perf/src/main.rs perf/Cargo.toml
	cd perf && cargo build --release

docker-build-standalone:
	docker build -t gwbasic -f Dockerfile.standalone .

docker-build-httpd:
	docker build -t gwbasic-httpd -f Dockerfile.httpd .

clean:
	rm -rf perf/target

run-hello-dos:
	GWBASIC=$(GWBASIC_EXE) ruby run-dos-box.rb ./src/HELLO.BAS

run-hello-docker: docker-build-standalone
	docker.exe run --rm -v $(PWD_UNIX)/src:/basic/src -v $(PWD_UNIX)/bin:/basic/bin gwbasic HELLO.BAS

run-httpd: docker-build-httpd
	docker run --rm -d --name gwbasic-httpd -v $(PWD_UNIX)/rest:/basic/src -v $(PWD_UNIX)/bin:/basic/bin -p 8080:80 gwbasic-httpd
	curl http://localhost:8080/api/todo
	curl --data "Hello world" -H "Content-Type: text/plain" http://localhost:8080/api/todo
	curl http://localhost:8080/api/todo
	docker stop gwbasic-httpd

test: run-hello-dos run-hello-docker run-httpd
