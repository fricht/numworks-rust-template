# Numworks Rust Template

A template with some tools to start creating your own rust [Numworks](https://www.numworks.com/) app.

## How to use

Create a new repo using this template.

### Method 1
> More convenient but requires `npm`

You need to have `npm` install with the [nwlink](https://www.npmjs.com/package/nwlink) package.

Just run `cargo run` with your calculator connected and that's it !

### Method 2
> Doesn't require `npm` but can't create app icon

Run `cargo build`, it will generate an app in `target/thumbv7em-none-eabihf/debug/`.
You can install it [here](https://my.numworks.com/apps).

## Futur improvments

- [ ] Rewrite functions documentation, clean up code.
- [ ] Finish full trigonometric functions approximations, and improve existing ones (Taylor ?).
- [ ] Write a complete panic handler.
- [ ] Finish the `Color` struct implementations in `eadk.rs`.
- [X] Add a way to compile the app into a .nwa file, to share it.

Feel free to report issues or ask for new features.

Good luck !
