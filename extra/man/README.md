# Rio Terminal Man Pages

This directory contains manual pages for Rio terminal emulator in scdoc format.

## Files

- `win.1.scd` - Main Rio terminal manual page (section 1)
- `win.5.scd` - Rio configuration file format manual page (section 5)  
- `win-bindings.5.scd` - Rio key bindings manual page (section 5)

## Building

To build the man pages, you need `scdoc` installed:

### Install scdoc

**macOS (Homebrew):**
```bash
brew install scdoc
```

**Ubuntu/Debian:**
```bash
sudo apt install scdoc
```

**Arch Linux:**
```bash
sudo pacman -S scdoc
```

**From source:**
```bash
git clone https://git.sr.ht/~sircmpwn/scdoc
cd scdoc
make
sudo make install
```

### Build man pages

```bash
# Build all man pages
make -C extra/man

# Or build individually
scdoc < extra/man/win.1.scd > win.1
scdoc < extra/man/win.5.scd > win.5
scdoc < extra/man/win-bindings.5.scd > win-bindings.5
```

### Install man pages

```bash
# Install to system man directory (requires sudo)
sudo cp win.1 /usr/local/share/man/man1/
sudo cp win.5 /usr/local/share/man/man5/
sudo cp win-bindings.5 /usr/local/share/man/man5/

# Update man database
sudo mandb
```

### View man pages

```bash
man win
man 5 win
man 5 win-bindings
```

## Format

The man pages are written in scdoc format, which is a simple markup language for writing man pages. See the [scdoc documentation](https://git.sr.ht/~sircmpwn/scdoc) for syntax details.