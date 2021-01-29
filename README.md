<p align="center">
  <img src="https://parry.rs/img/logo_parry.svg" alt="crates.io">
</p>
<p align="center">
    <a href="https://discord.gg/vt9DJSW">
        <img src="https://img.shields.io/discord/507548572338880513.svg?logo=discord&colorB=7289DA">
    </a>
    <a href="https://crates.io/crates/parry2d">
         <img src="http://meritbadge.herokuapp.com/parry2d?style=flat-square" alt="crates.io">
    </a>
    <a href="https://crates.io/crates/parry3d">
         <img src="http://meritbadge.herokuapp.com/parry3d?style=flat-square" alt="crates.io">
    </a>
</p>
<p align = "center">
    <strong>
        <a href="http://docs.rs/parry2d">2D Documentation</a> | <a href="http://docs.rs/parry3d">3D Documentation</a> | <a href="http://parry.rs">User Guide</a>
    </strong>
</p>

parry
========

**parry** is a 2 and 3-dimensional collision detection library written with
the rust programming language.

The official user guide is available [here](http://parry.rs).
The rustdoc documentation is available [for 3D](http://parry.rs/rustdoc/parry3d) and [for 2D](http://parry.rs/rustdoc/parry2d).

## Compilation
You will need the last stable build of the [rust compiler](http://www.rust-lang.org)
and the official package manager: [cargo](https://github.com/rust-lang/cargo).

Simply add one the following (or both) to your `Cargo.toml` file:

```
[dependencies]
parry2d = "0.23" # For 2D collision detection.
parry3d = "0.23" # For 3D collision detection.
```


## Features
- dynamic bounding volume tree based broad phase
- ball vs. ball collision detection,
- plane vs. any convex object collision detection.
- collision detection between arbitrary convex objects
- compound geometries
- ray-casting
- time of impact computation  for objects without rotational movement (compound vs. compound is not
  yet implemented)

And various traits for collision detectors and broad phase collision detection.
