# Numworks Rust Template

A template with some tools to start creating your own rust [Numworks](https://www.numworks.com/) app.

## How to use

Create a new repo using this template.

### Method 1
> More convenient but requires `npm`

You need to have `npm` install with the [nwlink](https://www.npmjs.com/package/nwlink) package.

Just run `cargo run` with your calculator connected and that's it !

### Method 2
> Doesn't require `npm`

Run `cargo build`, it will generate an app in `./target/thumbv7em-none-eabihf/debug/`.
Note that the generated file won't add any extension. You can manually add `.nwa` to make it prettier.
Then you can install it on your calculator [here](https://my.numworks.com/apps).


Feel free to report issues or ask for new features.

Good luck !
