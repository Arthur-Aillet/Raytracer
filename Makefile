##
## Project : Rustracer
## File : Makefile
##

NAME = rustracer

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

.PHONY: all clean   fclean  re
