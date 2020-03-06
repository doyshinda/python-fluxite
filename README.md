# python-fluxite

This is a Python wrapper (using [PyO3](https://github.com/PyO3/pyo3)) around the [fluxite](https://github.com/doyshinda/fluxite) library, used for sending metrics formatted for InfluxDB or Graphite over UDP.

## Usage
 * Install Rust nightly: `rustup install nightly`
 * Set this directory to use nightly: `rustup override set nightly`
 * Build: `cargo build --release`
 * Rename the lib file in `target/release`:
    * on MacOS, rename `libpython_fluxite.dylib` to `python_fluxite.so`
    * on Windows `libpython_fluxite.dll` to `python_fluxite.pyd`
    * on Linux `libpython_fluxite.so` to `python_fluxite.so`
 * Copy the lib to where you want to use it:
```Python
>>> import python_fluxite as metrics
>>> metrics.initialize_reporter(endpoint='localhost:12345', prefix='app=my_app', observer_type='influx')
True
>>> metrics.timer('test', 100)
True
>>> metrics.count('foo', 1)
True
```
will yield metrics like:
```
test,app=my_app min=100,p50=100,p75=100,p99=100,max=100 1583532538416379278
foo,app=my_app count=1 1583532540419673108
```

With Graphite:
```Python
>>> import python_fluxite as metrics
>>> metrics.initialize_reporter(endpoint='localhost:12345', prefix='my_app', observer_type='graphite')
True
>>> metrics.timer('test', 100)
True
>>> metrics.count('foo', 1)
True
```
metrics look like this:
```
my_app.test.min 100 1583533026370819708
my_app.test.p50 100 1583533026370819708
my_app.test.p75 100 1583533026370819708
my_app.test.p99 100 1583533026370819708
my_app.test.max 100 1583533026370819708
my_app.foo 1 1583533028371115264
```