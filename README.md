# My Rust Game



## Installation / Getting Started:

(project currently incomplete)
To start the project, first make sure to have the listed dependencies.  Then clone this repo and navigate to the root directory (directory this file is in).

Then run the following command:

```bash
cargo run
```


## Dependencies:

- Cargo (rust package manager)


#### For Linux Users

You might need to install some dependencies. Please refer to [this section](https://github.com/amethyst/amethyst#dependencies) of the README for more details.

## Features

This project contains the minimum amount of code needed to draw sprites to the screen. Here's a small summary of what you'll find in the source files:

- `resources/display_config.ron`  
  Contains the window configuration (size, title).

- `src/main.rs`  
  Creates the render graph, adds the required bundles, builds the game data with our own state and finally, starts the game's main event loop.

- `src/state.rs`  
  Implements the main game state. In the `on_start` hook, the camera is initialized, and the sprites that will be drawn are loaded and their entities created.  
   In the `handle_event` hook, we print any keys that were pressed and close the window if the user presses escape or the OS requests that we quit.
