# marinade_finance_interface

Generated with [solores](https://github.com/igneous-labs/solores) v0.1.2


```
solores --output-dir ../ idl.json
```

IDL downloaded from https://github.com/marinade-finance/marinade-ts-sdk/blob/main/src/programs/idl/marinade-finance-idl.json

## Notes

- original idl.json contained some typedefs that weren't in the src code: `enum CommonError` (different from the one in `errors.rs`) and `enum InitializeError`. These have been removed.
- original idl.json did not contain program address `MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD`