# Ctags Sys

- [x] make compile success
- [x] callable
- [ ] use packcc to convert code kotlin.peg and varlink.peg
- [ ] remove main `exit(0)`

## setup

other requirements: [packcc](https://github.com/arithy/packcc)

### Setup

see in ctags workflow

macOS:

```
brew install automake pkg-config bash libxml2 jansson libyaml gdb docutils
```

GNU/Linux

```bash
apt install valgrind pkg-config automake bash libjansson-dev libyaml-dev libseccomp-dev libxml2-dev gdb
```

Windows MSYS

```bash
pacman -S --noconfirm gcc libiconv-devel pkg-config python dos2unix
```

## logs

```
gcc -DHAVE_CONFIG_H -I.  -I. -I. -I./main -I./peg -DHAVE_PACKCC  -DUSE_SYSTEM_STRNLEN   -DHAVE_REPOINFO_H  -std=gnu99 -Wall    -I/usr/local/Cellar/jansson/2.13.1/include -I/usr/local/Cellar/libyaml/0.2.5/include 
```

```
gcc -DHAVE_CONFIG_H -I.  -I. -I. -I./main -I./peg -DHAVE_PACKCC  -DUSE_SYSTEM_STRNLEN   -DHAVE_REPOINFO_H  -std=gnu99 -Wall  -I/usr/local/Cellar/libyaml/0.2.5/include  -g -O2 main/args.c
-I/usr/local/Cellar/jansson/2.13.1/include
```
