##
## Project : Rustracer
## File : Makefile
##

NAME = raytracer

all: $(NAME)

$(NAME):
	cargo build
	cp target/debug/$(NAME) .

run:
	cargo run

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all

test:
	cargo test -- --nocapture

.PHONY: all clean test fclean  re
