pwl
===

[![Build Status](https://dev.azure.com/yskszk63/pwl/_apis/build/status/yskszk63.pwl?branchName=master)](https://dev.azure.com/yskszk63/pwl/_build/latest?definitionId=1&branchName=master)
[![codecov](https://codecov.io/gh/yskszk63/pwl/branch/master/graph/badge.svg)](https://codecov.io/gh/yskszk63/pwl)

my Powerline by Rust (for my study)

Getting started
---------------

Bash

```
.bashrc
------------------
...
prompt() { PS1="$(path/to/pwl $?)"; }
PROMPT_COMMAND=prompt
```
