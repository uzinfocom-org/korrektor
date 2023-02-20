**How to get Korrektor/Qalam working**

Make sure you have following installed on your machine:

- pcre
- ispell
- aspell
- hunspell

**Debian, Ubuntu**

`sudo apt-get install libpcre3 libpcre3-dev hunspell aspell ispell`
`sudo ln -sf /usr/lib/x86_64-linux-gnu/libpcre.so /usr/lib/x86_64-linux-gnu/libpcre.so.1`

**Arch Linux**

`sudo pacman -S pcre hunspell ispell aspell`

**MacOS**

`brew install pcre hunspell ispell aspell`

Also, don't forget to install hunspell data! It can be done by executing:

`wget ...`
