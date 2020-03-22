Chudnovsky (Rust-Python demo project)
=====================================

This package contains a naive implementation of the [Chudnovsky
algorithm](https://en.wikipedia.org/wiki/Chudnovsky_algorithm) to compute pi to
a given number of digits. This implementation is not meant to be used for any
other purposes than demonstration.

Requirements
------------

* Python 3.5 or up
* a sufficiently recent version of pip
* rust, with the nightly toolchain activated

Build with `pip install . -v`, or use maturin directly (`maturin develop`).

Usage
-----

```python
>>> from chudnovsky import compute_pi
>>> compute_pi(1000, 77)
'3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196442881097566593344612847564823378678316527120190914564856692346034861045432664821339360726024914127434'

```
