# Password Generator(pwgn)
## Install
```
$ cargo install --git https://github.com/hurou927/password_generator.git
```

## Usage
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
❯ pwgn -l 20 -N 5 | jq .
{
  "version": "1.0.0",
  "passwords": [
    {
      "text": "9<-oBM\"f(jD}:{jTub2["
    },
    {
      "text": "k4:rjW[HNaY)x4>%r-WL"
    },
    {
      "text": "*ZN\\|V@y65j7.6qvX450"
    },
    {
      "text": "l_;6YR^w+;@I9.BH0Fyd"
    },
    {
      "text": "=pjW{lK#5!m(S\\Q%4AjZ"
    }
  ]
}
```
