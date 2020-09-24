# JSON Schema hacking toolset

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-brightgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Package](https://img.shields.io/crates/v/jsonst.svg?style=popout)](https://crates.io/crates/jsonst)
[![Docs.rs API Docs](https://docs.rs/jsonst/badge.svg)](https://docs.rs/jsonst)

This is all-in-one toolset for __JSON Schema__.

## Usecases

This tools allow you:

- Search schemas by pattern on [schemastore.org](https://schemastore.org/)
- Retrieve found schema from _schemastore_
- Infer schema from data-sample to help start hacking it (thanks to [infers-jsonschema](https://github.com/Stranger6667/infers-jsonschema))
- [TODO] Optimize existing schema (thanks to [jsonschema-equivalent](https://github.com/macisamuele/jsonschema-equivalent))
- Validate existing data using schema from file or from _schemastore_ using one of supported validator

## Configuration

A rich set of features allows costomize this tool before build.

- Support multiple JSON Schema validators (`feature = "all-validators"` enables all)
  - [valico](https://crates.io/crates/valico) (draft-6 only, `feature = "valico"`)
  - [jsonschema](https://crates.io/crates/jsonschema) (draft-4/6/7, `feature = "jsonschema"`)
  - [jsonschema-valid](https://crates.io/crates/jsonschema-valid) (draft-4/6/7, `feature = "jsonschema-valid"`)
- Support many input data formats (`feature = "all-parsers"` enables all)
  - Text formats (`feature = "txt-parsers"` enables all)
    - json (default)
    - json5 (`feature = "json5"`)
    - yaml (`feature = "yaml"`)
    - toml (`feature = "toml"`)
    - ron (`feature = "ron"`)
  - Binary formats (`feature = "bin-parsers"` enable all)
    - bson (`feature = "bson"`)
    - cbor (`feature = "cbor"`)
    - pickle (`feature = "pickle"`)
- Integration with [schemastore.org](https://schemastore.org/) (`feature = "schemastore"`)
  - Adds `search` command which allows find schemas by patterns
  - Adds `retrieve` command which download schema from store
- An `infer` command which can help infer JSON Schema from data (`feature = "infers"`)

## Validators comparison

| Validator        | Pros                     | Cons                  |
| ---------        | ----                     | ----                  |
| valico           | Meaningful error reports | Slow validation       |
| jsonschema       | Fast validation          | Obscure error reports |
