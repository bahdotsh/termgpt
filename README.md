<div align="center">
	<br>
	<br>
	<h1>TermGPT</h1>
	<br>

Interact with ChatGPT from your terminal! 🚀🤖

</div>


## Install

### Cargo

```bash
cargo install termgpt
termgpt --help
```

### From source

```bash
git clone git@github.com:bahdotsh/termgpt.git
cd termgpt
cargo install --path .
```

## Usage

For the first time you have to to enter your [OpenAI API key](https://platform.openai.com/account/api-keys). 

```
termgpt --api <your api-key>
```
This key will then be saved to: 
```
// Linux:   /home/alice/.config/termgpt
// Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\termgpt
// macOS:   /Users/Alice/Library/Application Support/termgpt
```
To be reused later

### Questions

For now you can ask ChatGPT anything related to managing your operating system or writing code. Eventually you'll be able to ask any question to ChatGPT (couple more prompts to go) with termgpt (P.S try to avoid using '?' at the end of your questions for now).

```bash
termgpt <any question>
```

### Executing commands 

You can ask to execute the generated command using `--exec` argument.

```bash
termgpt --exec list all the files in this folder 
```

### Help message

```bash
termgpt --help


A cli tool to interact with ChatGPT

Usage: termgpt [OPTIONS] [CHAT]...

Arguments:
  [CHAT]...

Options:
      --api <API>  Your API key for OpenAPI
  -e, --exec       Execute the commands generated
  -h, --help       Print help
  -V, --version    Print version
```


## Example questions
```
termgpt How do I list all files in the current directory
termgpt How can I find the number of lines in a file
termgpt How do I create a new directory
termgpt How can I check the available disk space
termgpt How do I delete a file
termgpt How can I search for a specific string in a file
termgpt How do I copy a file to a different location
termgpt How can I see the contents of a file
termgpt How do I move a file to a different location?
termgpt How can I change the permissions of a file or directory
```

## Inspirations

- [shell-gpt](https://github.com/TheR1D/shell_gpt)
- [shell-gpt-rs](https://github.com/rigwild/shell-gpt-rs)

## License

The MIT license
