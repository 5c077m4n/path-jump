# Path Jump

A tiny cli tool to help you jump to the dir you want based on your history

### Installation

1. Go to [https://github.com/5c077m4n/path-jump/releases](https://github.com/5c077m4n/path-jump/releases) and download the right zip/tar file for you
1. Add files to `${XDG_DATA_HOME:-$HOME/.local/data}` or whereever
1. `chmod +x /path/to/j` (if needed)
1. Add to your `.zshrc`/`.bashrc` file:
	```bash
	(cd /path/to/j && source ./j.sh)
	```
1. `cd` your way to where ever
1. `j` your way to happyness!

### Usage

Just press `j` to jump to whatever directory you want:
- If you want to a relative path then append at the start `./`
- If you want to use one from your history the just press `j` followed by whatever you remember from the path

### TODO

- [ ] Make a rusty api
- [ ] Add tab completion
- [ ] Add a dialog to show top relevant options
- [ ] Any ideas?

---

Inspired by [z](https://github.com/rupa/z/).
