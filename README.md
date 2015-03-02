compass
===


THIS IS WORK IN PROGRESS!

Stand alone command line executable using sass-rs.

The image-width and image-height functions are implemented, however only one of them is expanded. See example below.

```
cargo run data/image-functions.scss
     Running `target/compass data/image-functions.scss`
Compiling sass file: `data/image-functions.scss`.
Entering image_height
------- css  ------
body {
  width: image-width("data/logo1.png");
  height: 64; }

--------
```
