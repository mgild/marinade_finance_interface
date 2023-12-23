# marinade_finance_interface

Generated with [solores](https://github.com/igneous-labs/solores) v0.5.0

To (re)generate, run in current directory:

```sh
solores \
    -o ../ \
    --solana-program-vers "^1" \
    --borsh-vers ">=0.9" \
    --thiserror-vers "^1" \
    --num-derive-vers ">=0.1" \
    --num-traits-vers ">=0.1" \
    --serde-vers "^1" \
    idl.json
```

IDL downloaded from https://github.com/marinade-finance/marinade-ts-sdk/blob/main/src/programs/idl/json/marinade_finance.json

## Notes

- original idl.json did not contain program address `MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD`
- original idl.json was missing `msg` field on `NotUsed6027`