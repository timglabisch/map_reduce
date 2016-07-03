build_reducer:
	cd reducer && cargo build --release && cd ..

build_mapper:
	cd mapper && cargo build --release && cd ..

build_producer:
	cd producer && cargo build --release && cd ..

build: build_reducer build_mapper build_producer

run_rust: build
	time ./producer/target/release/producer 1000 | ./mapper/target/release/mapper | sort | ./reducer/target/release/reducer

run_php:
	time ./producer/target/release/producer 1000 | php mapper.php | sort | php reducer.php
