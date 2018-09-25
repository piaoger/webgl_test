# WebGL + Rust

because why not, i guess

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for Rust <-->
  JavaScript interop
* [`nalgebra`](http://nalgebra.org/) for linear algebra nonsense
* [`nphysics`](http://nphysics.org/) as a physics engine
* [`lazy-static.rs`](https://github.com/rust-lang-nursery/lazy-static.rs) for
  keeping global state
* [`actix-web`](https://actix.rs/) for the server side
* [`bincode`](https://github.com/TyOverby/bincode) for
  serialization/deserialization over the network
* [`pcg_rand`](https://github.com/robojeb/pcg_rand) for generating very
  high-quality random numbers very quickly

## try

```bash
git clone https://github.com/AugmentedFifth/webgl_test.git
cd webgl_test

curdir=$(pwd)
echo ${curdir}
${curdir}/bin/build.sh fresh

cd ${curdir}/server
make
cd ${curdir}/www/dist

killall -9 webgl_test_server
${curdir}/server/target/release/webgl_test_server &
cd ${curdir}

firefox localhost:11484
```

## license

![GNU Affero General Public License version 3+](https://www.gnu.org/graphics/agplv3-155x51.png)
