# `@trezoa/tpl-token-group`

A TypeScript interface describing the instructions required for a program to implement to be considered a "token-group" program for TPL token mints. The interface can be implemented by any program.

## Links

- [TypeScript Docs](https://trzledgerfoundation.github.io/trezoa-program-library/token-group/js/)
- [FAQs (Frequently Asked Questions)](#faqs)
- [Install](#install)
- [Build from Source](#build-from-source)

## FAQs

### How can I get support?

Please ask questions in the Trezoa Stack Exchange: https://trezoa.stackexchange.com/

If you've found a bug or you'd like to request a feature, please
[open an issue](https://github.com/trzledgerfoundation/trezoa-program-library/issues/new).

## Install

```shell
npm install --save @trezoa/tpl-token-group @trezoa/web3.js@1
```
_OR_
```shell
yarn add @trezoa/tpl-token-group @trezoa/web3.js@1
```

## Build from Source

0. Prerequisites

* Node 16+
* NPM 8+

1. Clone the trezoa:
```shell
git clone https://github.com/trzledgerfoundation/trezoa-program-library.git
```

2. Navigate to the library:
```shell
cd trezoa-program-library/token-group/js
```

3. Install the dependencies:
```shell
npm install
```

4. Build the library:
```shell
npm run build
```

5. Build the on-chain programs:
```shell
npm run test:build-programs
```
