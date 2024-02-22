# Rusty Snake Game

This is a simple snake game written in Rust.

The project was only possible with the help of this [repo](https://github.com/redox-os/games), with great examples
it was easy to implement the base of the game. 

As a side note: This is not a total ripoff of the snake game provided in the examples. I tried my best to not follow it, 
and only wrote code that was deemed necessary for the learning experience.

## Example

![output](https://github.com/Hereugo/rusty_snake_game/assets/60090566/f2eecdb1-c267-4202-b20f-e5a74b91c702)

## Author

- Amir Nurmukhambetov [github](https://github.com/Hereugo) 

## TODO

- [x] Create a struct Player. 
  - [x] Movement functionality
  - [x] Head coordinate
  - [x] Body coordinates
  - [x] Display method
  - [x] Update method
  - [x] Handle user input
- [x] Randomly generated apple. 
- [x] Create a game loop


## How to use:

First, you'll want to check out this repository

```shell
git clone https://github.com/hereugo/rusty_snake_game.git
```

With cargo already installed, you can simply run:

```shell
cargo build --release
```

Then run the program:

```shell
./target/release/rusty_snake_game # or .\target\release\rusty_snake_game.exe on Windows
```
