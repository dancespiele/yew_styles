# Yew Styles

<div align="center">
    <p>
        <a href="https://crates.io/crates/yew_styles"><img alt="Crate Info" src="https://img.shields.io/badge/yew__styles-framework%20styles-brightgreen"/></a>
        <a href="https://docs.rs/yew_styles/"><img alt="API Docs" src="https://img.shields.io/badge/yew__styles-docs-informational"/></a>
        <a href="https://discord.gg/VQck8X4"><img alt="Discord Chat" src="https://img.shields.io/discord/701068342760570933"/></a>
    </p>
</div>


Yew Styles is a style framework for yew

## Motivation
The purpose of developing this project is first,
provide a style framework for yew because there isn't not many options currently,
also to create a layout system which is not far of the flexbox concept, and,
to take the rust benefits and implement a properties selected by enumeration
in the most of the cases which makes fast for developing applications and avoids the practice try and error

## How it works
Each component is split in two parts, the logical yew component and its sass module,
however, it is not necessary to worry about the sass module only it needs to be include in the project
### How install it
1. Install the sass module: `npm install yew-styles`
2. Add the yew_style crate with the features needed for your project in Cargo.toml file: 
```toml
yew_styles = { version="0.6.0", features=["button", "navbar", "layouts"] }
```
3. Import the main.css file in you main javascript/typescript file project: 
```typescript
    import 'node_modules/yew-styles/main.css';
```
4. Ready to import and use in your project 🚀

## Run the documentation page
1. `git clone https://github.com/spielrs/yew_styles.git`
2. `cd yew_styles`
3. `npm start`

In the left side there is a list of links where each one access to a correspondent component documentation,
there, shows how to use it.

## Rust Docs

You can also see all the yewstyle documentation in rust docs [here](https://docs.rs/crate/yew_styles).
It include description and examples for each component

## Run the tests
Inside of the project run:

`cargo test --target wasm32-unknown-unknown --manifest-path=crate/yew_styles/Cargo.toml`

## Development phase
Yew style is in early phase, currently doesn't have enough components to cover all the requirements that could need a website/web application.
All contributions are appreciated.

## How contribute
First, open an issue describing about the fix, improvement or implementation and as suggestion, don't start to work in it until that is discussed.
If the contribution is a fix or small improvement in a component, only a pull request to master explaining what resolve or improve that, is required.
If it is an implementation, please follow the next requirements:

* Firstable open and issue describing about the component 
* Unit tests, which checks that the component is created and
its logic works, in the same file where it is implemented (test events is not needed for now)
* One component per file, if multiple components have connections between them, it is possible create subfolder
* Documentation in the component showing an example of using it and small description of each prop
* Create a component page in `/crate/src/page` with the same structure than the rest of the components

## Code of Conduct
Please check our [code of conduct](CODE_OF_CONDUCT.md)

## Roadmap

- [x] Button 
- [x] Layout
- [x] Navbar
- [x] Form
- [x] Card
- [ ] Text
- [ ] Table
- [ ] Pagination
- [x] Modal
- [ ] Sidebar
- [ ] Tab
- [ ] Tooltips
- [ ] Calendar
- [ ] Assets


## License

Yew Style is [MIT](LICENSE-MIT.md) and [Apache-2.0](LICENSE-APACHE.md) licensed
