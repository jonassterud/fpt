<picture>
  <source width='150px' srcset='media/logo/vector/default-monochrome-white.svg' media='(prefers-color-scheme: dark)'>
  <img width='150px' src='media/logo/vector/default-monochrome-black.svg'>
</picture>

 
\- a financial portfolio tracker.

[![CI](https://github.com/jonassterud/fpt/actions/workflows/ci.yml/badge.svg)](https://github.com/jonassterud/fpt/actions/workflows/ci.yml)
[![Release](https://github.com/jonassterud/fpt/actions/workflows/release.yml/badge.svg)](https://github.com/jonassterud/fpt/actions/workflows/release.yml)

## Supported platforms/assets
* Sparebank 1
* Nordnet (*coming soon*)
* Coinbase (*coming soon*)
* Bitcoin
* Ethereum (*coming soon*)

## Configuration
Configuration is done in the `config.toml` file that is created when you run the program.  
Remember that this file contains very sensitive data!
```toml
SPAREBANK1_ID = ''
SPAREBANK1_SECRET = ''
SPAREBANK1_REFRESH_TOKEN = ''
BITCOIN_ADDRESSES = ['']
```

## Contributing
Feel free to contribute!

Use tools such as [Rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) to improve your code.  
Commit messages should follow [conventionalcommits.org](https://www.conventionalcommits.org).  
Where type is one of the following: `feat`, `fix`, `ci`, `docs` or `refactor`.

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details.
