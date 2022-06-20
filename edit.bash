#!/bin/bash

set -euo pipefail

todo_source=$(fd 'TODO' 'docs')
main_source=$(fd 'main.rs' 'src')
sources=$(fd '.rs' --exclude 'main.rs' 'src')

$EDITOR ${todo_source} ${main_source} ${sources}
