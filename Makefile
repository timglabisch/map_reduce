crosscompile:
	rm -rf mv $(shell pwd)/reducer/target
	rm -rf mv $(shell pwd)/mapper/target
	rm -rf mv $(shell pwd)/producer/target
	rm -rf $(shell pwd)/reducer/release_linux
	rm -rf $(shell pwd)/mapper/release_linux
	rm -rf $(shell pwd)/producer/release_linux 
	cd reducer && rm -rf target && docker run -it --rm -v $(shell pwd)/reducer:/source jimmycuadra/rust cargo build --release
	cd mapper && rm -rf target && docker run -it --rm -v $(shell pwd)/mapper:/source jimmycuadra/rust cargo build --release
	cd producer && rm -rf target && docker run -it --rm -v $(shell pwd)/producer:/source jimmycuadra/rust cargo build --release
	mv $(shell pwd)/reducer/target/release $(shell pwd)/reducer/release_linux
	mv $(shell pwd)/mapper/target/release $(shell pwd)/mapper/release_linux
	mv $(shell pwd)/producer/target/release $(shell pwd)/producer/release_linux

build_reducer:
	cd reducer && cargo build --release && cd ..

build_mapper:
	cd mapper && cargo build --release && cd ..

build_producer:
	cd producer && cargo build --release && cd ..

build: crosscompile build_reducer build_mapper build_producer

run_rust: build
	time ./producer/target/release/producer 1000 | ./mapper/target/release/mapper | sort | ./reducer/target/release/reducer

run_php:
	time ./producer/target/release/producer 1000 | php mapper.php | sort | php reducer.php
