#!/bin/bash

OLDPWD=tmp/example1 && cargo build && cd - && ../../target/debug/regex-file-mover "$@" && cd - > /dev/null
