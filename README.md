<div align="center">
	<br>
	<br>
	<h1>RGPT</h1>
	<br>

Interact with ChatGPT from your terminal! 🚀🤖

</div>


## Install

### Cargo

```bash
cargo install rgpt
rgpt --help
```

### From source

```bash
git clone git@github.com:bahdotsh/rgpt.git
cd rgpt
cargo install --path .
```

## Usage

For the first time you have to to enter your [OpenAI API key](https://platform.openai.com/account/api-keys). 

```
rgpt --api <your api-key>
```
This key will then be saved to 
  // Linux:   /home/alice/.config/rgpt
  // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\rgpt
  // macOS:   /Users/Alice/Library/Application Support/rgpt

To be reused later

### Questions

For now you can ask ChatGPT anything related to managing your operating system or writing code. Eventually you'll be able to ask any question to ChatGPT (couple more prompts to go) with rgpt (P.S try to avoid using '?' at the end of your questions for now).

```bash
rgpt <any question>
```

### Executing commands 

You can ask to execute the generated command using `--exec` argument.

```bash
rgpt --exec list all the files in this folder 
```

### Help message

```bash
rgpt --help
```


## Example questions
```
rgpt How do I list all files in the current directory
rgpt How can I find the number of lines in a file
rgpt How do I create a new directory
rgpt How can I check the available disk space
rgpt How do I delete a file
rgpt How can I search for a specific string in a file
rgpt How do I copy a file to a different location
rgpt How can I see the contents of a file
rgpt How do I move a file to a different location?
rgpt How can I change the permissions of a file or directory
```

## Inspirations

- [shell-gpt](https://github.com/TheR1D/shell_gpt)
- [shell-gpt-rs](https://github.com/rigwild/shell-gpt-rs)

## License

The MIT license
