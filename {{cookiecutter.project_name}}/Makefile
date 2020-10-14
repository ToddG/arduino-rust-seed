.PHONY: help
help:
	# -----------------------------------------------------------------------------
	# Targets:
	#
	#	build		: run `cargo build` for each crate
	#	clean 		: run `cargo clean` for each crate
	#	deploy		: use `avrdude` to deploy mseg-bin to device
	#	help 		: show this message
	#
	# end.
	# -----------------------------------------------------------------------------


.PHONY: clean
clean:
	(cd {{cookiecutter.project_name}}-bin && cargo clean)

.PHONY: build
build:
	(cd {{cookiecutter.project_name}}-bin && cargo build)

.PHONY: deploy
deploy:
	(cd {{cookiecutter.project_name}}-bin && avrdude -v -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/{{cookiecutter.project_name}}-bin.elf:e")
