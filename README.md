## Simple lexical analyzer built in Rust. 

This is a simple lexical analyzer built in Rust. It reads a file and tokenizes it. The tokens are then printed to the console.

The tokens are defined in the `token.rs` file

## Sample token file

```
// A comment should be ignored
class Result {
    fun hello() {
        var result = (a + b) > 7 and "Success" != "Failure" or x >= 5;

        while (result) {
            var counter = 0;
            counter = counter + 1;
            if (counter == 10) {
                return nil;
            }
        }

        return result;
    }
}
```

## Usage

```sh
$ ./your_program_name tokenize <file>
```
