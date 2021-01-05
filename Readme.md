# Mini-Grep

## **Overview**

Project from "[The Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)", slightly modified.

## **Features**

The application checks text by each new line. The terminal then returns output of the corresponding Query term and the line it resides on.

## **Running the project**

### **Startup**

```rust
    cargo install
```

### **Case Sensitive Search (Default**)

```rust
    cargo run QUERY_TERM FILENAME.FILE_EXTENSION
```

1. QUERY_TERM:

   - The query term is searched against all text within the specified file.
   - After the QUERY_TERM is found the program retruns a line with the specified text.

2. FILENAME.FILE_EXTENSION

   - This is the file that is being queried against the QUERY_TERM.

#### **Example of Case Sensitive Search**

- QUERY_TERM = foo

- FILENAME.FILE_EXTENSION = foobar.txt

  ```zsh
    "\
    Hello World!
    My name is foo bar,
    and I love programming."
  ```

- Command:

  ```zsh
    cargo run foo foobar.txt
  ```

The Output becomes: "My name is foo bar,"

### **Case Insensitive Search**

Case Insensitive Search (C.I.S.) is similar to Case Sensitive Search (C.S.S.). Both of them use a QUERY_TERM and a FILENAME.FILE_EXTENSION as their Environment Arguments.

The difference between C.I.S. and C.S.S. is the addition of the optional argument "CASE_INSENSITIVE". The application checks whether or not it exists, and if it does then queries the text via Case Insensitivity.

Due to the varying ecosystems and the complexity of each systems API for env variables the way to do it for Windows is different then Linux/Mac.

#### **Linux/Mac**

The Terminal residing in Linux/Mac has a separate API for handling environment variables and has separate commands for interacting with the terminal.

```ZSH
    CASE_INSENSITIVE=1 cargo run QUERY_TERM FILENAME.FILE_EXTENSION
```

#### **Windows**

Windows allows you to set the environment variables via the `set` command.

##### **SETTING ENV**

```CMD
    set CASE_INSENSITIVE=1

    cargo run QUERY_TERM FILENAME.FILE_EXTENSION
```

OR

```CMD
    set CASE_INSENSITIVE=1 && cargo run QUERY_TERM FILENAME.FILE_EXTENSION
```

This sets the environment variable in the OS and allows the program to perform C.I.S. .

##### **UNSET ENV**

To unset the ENV variable set it with an empty value:

```CMD
    set CASE_INSENSITIVE=

    cargo run QUERY_TERM FILENAME.FILE_EXTENSION
```

OR

```CMD
    set CASE_INSENSITIVE= && cargo run QUERY_TERM FILENAME.FILE_EXTENSION
```

## Dependencies

Only dependency necessary for this is ansi_term. This is used to colorize the output of the terminal.
