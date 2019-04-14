# node-crossterm

> Node bindings for Rust's [crossterm](https://github.com/TimonPost/crossterm)

## Install

```sh
$ npm install node-crossterm
```

## Example

```js
const crossterm = require("node-crossterm");
crossterm.clear(crossterm.ClearType.All);
crossterm.goto(13, 37);
crossterm.pos();
```

## API

All methods of [Terminal](https://docs.rs/crossterm/0.6.0/crossterm/struct.Terminal.html) and [TerminalCursor](https://docs.rs/crossterm/0.6.0/crossterm/struct.TerminalCursor.html) are available as methods on the default export.

Other features of `crossterm` like AlternateScreen and Screen are not available in this library (yet).

### Cursor

#### `crossterm.pos()`

Get the terminal cursor position.

Returns object with `x` and `y` properties.

#### `crossterm.set_pos(x, y)`

Set terminal cursor position.

#### `crossterm.move_{up,down,right,left}(n)`

Move the terminal cursor by `n` in direction specified by function name.

#### `crossterm.save_position()`

Save the terminal cursor position.

#### `crossterm.reset_position()`

Restore the terminal cursor position.

#### `crossterm.hide()`

Hide the terminal cursor.

#### `crossterm.show()`

Show the terminal cursor.

#### `crossterm.blink(b)`

Set cursor blink on or off.

### Terminal

#### `crossterm.ClearType`

JavaScript equivalent of [crossterm ClearType enum](https://docs.rs/crossterm/0.6.0/crossterm/enum.ClearType.html).

```
{
    All,
    FromCursorDown,
    FromCursorUp,
    CurrentLine,
    UntilNewLine
}
```

#### `crossterm.clear(clear_type)`

Clear the terminal according to the given `clear_type`.

```js
// Example: clear all lines of the terminal
crossterm.clear(crossterm.ClearType.All);
```

#### `crossterm.terminal_size()`

Get the terminal size.

Returns object with `width` and `height` properties.

#### `crossterm.scroll_up(n)`

Scroll the terminal up `n` rows.

#### `crossterm.scroll_down(n)`

Scroll the terminal down `n` rows.

#### `crossterm.set_size(width, height)`

Set the terminal width and height.

#### `crossterm.exit()`

Exit the current process.

#### `crossterm.write(str)`

Write a string to the terminal.

## Terminal Support

From the [crossterm](https://github.com/TimonPost/crossterm#tested-terminals) docs:

> - Windows Powershell
>   - Windows 10 (pro)
> - Windows CMD
>   - Windows 10 (pro)
>   - Windows 8.1 (N)
> - Ubuntu Desktop Terminal
>   - Ubuntu 17.10
> - (Arch, Manjaro) KDE Konsole
> - Linux Mint

## License

MIT © [Sam Gluck](https://github.com/sdgluck)
