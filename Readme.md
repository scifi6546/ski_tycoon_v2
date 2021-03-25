# What will This Be? 
Summit Surveyor is a ski resort building game akin to Roller Coaster Tycoon. Build a sucessful enterprise while skillfully balancing the 
wants of pros and novices alike. Sculpt your terrain to appease customers. 
# Screenshots
![](/screenshots/screenshot.png)
# Marketing aside, What does the Game do Now?
Currently the game allows you to build lifts and simulates skiers skiing down slopes. The game runs on the web in a custom engine. Support for PC platforms is planned. 
# Playable Link
https://scifi6546.github.io/ski_tycoon_web_export/index.html
# Building the Game
## Prerequisite Software
```
cargo,
wasm-pack,
npm
```
## Build Instructions
To run the game fist install the rust toolchain by following the directions at https://www.rust-lang.org/. Next install wasm-pack using the instructions at https://rustwasm.github.io/wasm-pack/installer/.



Next clone this repo with
```
git clone https://github.com/scifi6546/ski_tycoon_v2.git
```
Then enter the directory ski_tycoon_v2/ski_tycoon
```
cd ski_tycoon_v2/ski_tycoon_v2
```
Next build the rust project (may take a while on slow computers)
```
wasm-pack build --release
```
Next enter the folder for the node js project.
```
cd ../www
```
Next install npn components
```
npm install
```
Finally either build the development environment with
```
npm run start
```
Or build a minimized project
```
npm run build
```
