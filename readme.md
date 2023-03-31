# Rusty Asteroids

![Demo Video](https://github.com/max-taylor/Rusty-Asteroids/blob/main/demo.gif)

Game written entirely in Rust to help me learn the language.

Using [crossterm](https://github.com/crossterm-rs/crossterm) to handling writing to stdout, allows you to specify coordinates to write characters and also colors.

Not using any game framework, simply creating 2D matrices to represent actors (player, asteroids, bullets) and using a simple game loop to update positions, respond to keyboard events and handle collisions.

Tried to follow ECS (entity component system) where possible.

# Game

To play the game, clone the repo and run: `cargo run`

## Bullets

Two bullet types:

- basic bullet
- spread bullet

A basic bullet gives 3 points per asteroid, whereas a spread bullet only gives 1 point.

## Controls

- Basic Bullet: `q`
- Spread Bullet: `<SPACE>`
- Exit game: `<ESC>`
- For ship controls use the arrow keys
