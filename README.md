# CLICEXLA
your silliest keyboard sound maker!
a simple program that gives silliness to every keypress mouse presss and scroll!
## Install!
### for nix!
```bash
git clone https://github.com/quotequack/clickexla/
cd clickexla
nix profile install .
```
### for linux!
(have rust and other things installed)
(list of dependencies in flake.nix!)
```
git clone https://github.com/quotequack/clickexla/
cd clickexla
cargo build --release
sudo cp ./target/release/clicer /usr/bin
```
### for others!
am sorry but like... figure it out and post what you did here as a pull request for others! (please)
## Usage!
run the program from your favourite application launcher or from the terminal! select your settings (in fields please use numbers) and after the program hides the sounds should work (settings are kept between sessions and are kept in .config/clickexa.json at least for linux)
To quit press ctrl+alt+q while the ui is hiden (after you click execute)
## For custom sounds
for custom sounds in the clickexla in .config (linux & macos), drop in a sound and rename it typing.mp3. After that you should be able to use customsound option within the app and get custom sounds
## Contribution 
Any and all contributions are HIGHLY appreciated
## Roadmap to 1.0
* Windows install guide and release
* Mac install guide and release
## Completed
* Custom sounds
* Application icon and .desktop app
* Json settings easy to access
* UI
* Different sound gen waves
* Sounds individually togglable
* Easy way to quit
