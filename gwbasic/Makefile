.PHONY: all run-perf perf docker-build-standalone docker-build-httpd clean

PERF_EXE = ./perf/target/release/perf.exe
PERF_COUNT = 1
GWBASIC_EXE = ./bin/GWBASIC.EXE
# TODO fix hardcoded unix PWD
PWD_UNIX = /c/Users/ngeor/Projects/github/dockerfiles/gwbasic

LAUNCHER_EXE = ./basic-launcher-rust/target/release/basic-launcher-rust.exe

all: build-perf build-launcher build-docker-standalone build-docker-httpd

run-perf: build-launcher build-perf
	GWBASIC=$(GWBASIC_EXE) $(PERF_EXE) --count $(PERF_COUNT)

build-perf: $(PERF_EXE)

$(PERF_EXE): perf/src/main.rs perf/Cargo.toml
	cd perf && cargo build --release

build-launcher: $(LAUNCHER_EXE)

$(LAUNCHER_EXE): $(wildcard basic-launcher-rust/src/*.rs) basic-launcher-rust/Cargo.toml
	cd basic-launcher-rust && cargo build --release

build-docker-standalone:
	docker build -t gwbasic -f Dockerfile.standalone .

build-docker-httpd:
	docker build -t gwbasic-httpd -f Dockerfile.httpd .

clean:
	rm -rf perf/target
	rm -rf basic-launcher-rust/target

run-hello-dos: build-launcher
	GWBASIC=$(GWBASIC_EXE) $(LAUNCHER_EXE) ./src/HELLO.BAS

run-hello-docker: build-docker-standalone
	docker.exe run --rm -v $(PWD_UNIX)/src:/basic/src -v $(PWD_UNIX)/bin:/basic/bin gwbasic HELLO.BAS

run-httpd: build-docker-httpd
	docker run --rm -d --name gwbasic-httpd -v $(PWD_UNIX)/rest:/basic/src -v $(PWD_UNIX)/bin:/basic/bin -p 8080:80 gwbasic-httpd
	curl http://localhost:8080/api/todo
	curl --data "Hello world" -H "Content-Type: text/plain" http://localhost:8080/api/todo
	curl http://localhost:8080/api/todo
	docker stop gwbasic-httpd

start-httpd-foreground: build-docker-httpd
	docker run -e NO_CLEANUP=1 --rm --name gwbasic-httpd -v $(PWD_UNIX)/rest:/basic/src -v $(PWD_UNIX)/bin:/basic/bin -p 8080:80 gwbasic-httpd

test: run-hello-dos run-hello-docker run-httpd
