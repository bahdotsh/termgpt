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

You can ask any question to ChatGPT with rgpt (try avoiding the use of '?' for now).

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


## Examples


## License

The MIT license
