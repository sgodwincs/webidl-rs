# webidl-parser

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://travis-ci.org/sgodwincs/webidl-parser.svg?branch=master)](https://travis-ci.org/sgodwincs/webidl-parser)

A parser for [WebIDL](https://heycam.github.io/webidl/) in Rust. Right now, it only includes a lexer, but a parser will added shortly. The lexer is done using [nom](https://github.com/Geal/nom) and so will the parser be.

Currently, the version of nom that is used is pulled from the repository. This is because `alt_complete!` is currently broken on the latest crates.io version which has not been updated with the master yet.
