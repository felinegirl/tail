
  

![Logo](https://github.com/felinegirl/tail/blob/main/gitlogo.png?raw=true)

Pretty much hammer writin in rust, so I can use it on linux :3
this is very very early development, to the point we don't even have rendering yet
I'm going to be monching on this project for a little bit

## Building

  

install le dependencies:

for a linux build you to get all these and exsrtact them to /cpp-include/linux

[sourcepp](https://github.com/craftablescience/sourcepp/releases/tag/v2026.1.3) (get "c-dist-ubuntu-latest.zip")
need to rename `sourcepp_vpkppc.os` to `libsourcepp_vpkppc.os`
also need to add `#include <stdbool.h>` to VPK.h

[miniz](https://github.com/richgel999/miniz)

[hat-trie](https://github.com/Tessil/hat-trie) (drop include folder on to /cpp-include/linux/include)

[half](https://github.com/craftablescience/sourcepp/blob/e205ad3f7f7f6f6e12a0963507f754722207fd70/ext/half/include/half.hpp) (just need to download drop half.hpp in to /cpp-include/linux/include)

I'll probably write a sh to automate this later
ps: not sure if its a arch linux thing, but build.rs or clang are not being nice; may have to mess around with the clang_args where the includes are


## Credits

  

- [Toy icons - my bunny :3](https://steamcommunity.com/id/meowingbunny/)

  

## TODO

- [ ] Set up properties
manly to just get fgd locations
also we need to write the settings some where
- [ ] Get webgpu render going
- [ ] implement importer/exporter vmf 
- [ ] figure out fgd and get that working
- [ ] implement vtf/vmf
- [ ] brushes
- [ ] vpks...... uuuugggg :skull: (this is going to be ass)
- [ ] mdl models
- [ ] add "physics engine" more for raycasting objects to select them

the more I write this list the more hope I loose... and this todo only has what I can think of aaaa
but it must be done