# Password Generator

```
Usage: password-generator [OPTIONS]

Options:
  -l <LENGTH>                        [default: 12]
  -a <LOWER_MIN_LENGTH>              [default: 1]
  -A <UPPER_MIN_LENGTH>              [default: 1]
  -s <SIMBOL_MIN_LENGTH>             [default: 1]
  -n <NUMBER_MIN_LENGTH>             [default: 1]
  -S, --symbol-chars <SIMBOL_CHARS>  [default: ~`!@#$%^&*()_-+={[}]|\:;"'<,>.?/]
  -N <NUM_PASSWORDS>                 [default: 1]
  -p, --preset <PRESET>              1: week password(half-width & number), otherwise: custom [default: 0]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Example

```sh
❯ cargo run -- -l 10 -N 10 | jq .
{
  "version": "1.0.0",
  "passwords": [
    {
      "text": "$E8OvG&-4z"
    },
    {
      "text": "LA[Rw\\_8B$"
    },
    {
      "text": "3dYd8HD*qz"
    },
    {
      "text": ",#7yWm{>dq"
    },
    {
      "text": "]EE~39<ps}"
    },
    {
      "text": "#;],6+X0l#"
    },
    {
      "text": "hHu.iGA~7E"
    },
    {
      "text": "D_E=5!p:U!"
    },
    {
      "text": "G=+1F*Sb~4"
    },
    {
      "text": "tUs4$ox\"^Q"
    }
  ]
}
```
