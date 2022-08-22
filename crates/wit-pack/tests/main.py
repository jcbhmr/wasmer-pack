#!/bin/env python3

from pathlib import Path
from typing import Union
from python.wit_pack import (
    load,
    Bindings,
    Exports,
    Module,
    Metadata,
    Abi,
    Error,
    Err,
    Ok,
    T,
    WitPack,
)

project_root = Path(__file__).parents[3]


def unwrap(value: Union[Ok[T], Err[Error]]) -> T:
    if isinstance(value, Err):
        raise Exception(value.value.verbose)
    else:
        return value.value


def load_bindings(wit_pack: WitPack) -> Bindings:
    metadata = Metadata.new(wit_pack, "wit-pack", "1.2.3")

    exports_wit = project_root.joinpath("crates", "wasm", "wit-pack.exports.wit")
    name = str(exports_wit)
    contents = exports_wit.read_text()
    exports = unwrap(Exports.from_wit(wit_pack, name, contents))

    wit_pack_wasm = project_root.joinpath(
        "target", "wasm32-unknown-unknown", "debug", "wit_pack_wasm.wasm"
    )
    module = Module.new(
        wit_pack, wit_pack_wasm.name, Abi.NONE, wit_pack_wasm.read_bytes()
    )

    return Bindings(metadata, exports, module)


def main():
    wit_pack = load()
    bindings = load_bindings(wit_pack)

    files = unwrap(wit_pack.generate_python(bindings))

    expected = {
        "wit_pack/wit_pack_wasm.wasm",
        "wit_pack/__init__.py",
        "setup.py",
        "wit_pack/bindings.py",
    }
    filenames = {f.filename for f in files}
    print(expected, filenames)

    assert filenames == expected


if __name__ == "__main__":
    main()
