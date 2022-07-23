# BAM!

A kinda functional programming language for the 3rd Lang Jam.

## Compiling And Installing

You compile and install Bam with cargo

    cargo install bam


The binary file will end up in `~/.cargo/bin`.

## Language Examples

Hello world:

    machine main() {
    	"Hello World"{1} -> stdout
    }


FizzBuzz:

    machine main() {
	    (1, 50) -> range -> x;
	    (x, 15) -> mod
	    ? "Fizzbuzz" ->stdout;
	    : (x, 3) -> mod;
	    ? "Fizz" -> stdout;
	    : (x, 3) -> mod
	    ? "Buzz -> stdout;
        : x -> stdout;
    }


Other examples are stored in the examples directory
