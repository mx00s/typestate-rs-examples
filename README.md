# [`typestate`] examples

See the [index of examples](./examples/README.md).

## Usage

Run the following, with `<name>` replaced by the name of the example, e.g. `door`.

```
EXAMPLE_NAME="<name>" make run
```

## Documentation

To see rendered documentation, including the state machine diagrams [`typestate`] emits, run

```
make docs
```

Then navigate to the module corresponding to the name of one of the examples, e.g. `door`.

## Diagram generation

[`typestate`]'s macro can generate diagrams.

```
make diagrams
```

[`typestate`]: https://github.com/rustype/typestate-rs
