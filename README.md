# Yew Styles

<div align="center">
    <p>
        <a href="https://yewstyles.spielrs.tech/"><img alt="Crate Info" src="https://img.shields.io/badge/Homepage-https%3A%2F%2Fyewstyles.spielrs.tech%2F-blue"/></a>
        <a href="https://crates.io/crates/yew_styles"><img alt="Crate Info" src="https://img.shields.io/badge/yew__styles-framework%20styles-brightgreen"/></a>
        <a href="https://docs.rs/yew_styles/"><img alt="API Docs" src="https://img.shields.io/badge/yew__styles-docs-informational"/></a>
        <a href="https://discord.gg/VQck8X4" target="_blank"><img alt="Discord Chat" src="https://img.shields.io/badge/Discor-Spielrs%20-yellowgreen"/></a>
        <a href="https://github.com/spielrs/yew_styles/blob/master/LICENSE-MIT.md" target="_blank"><img alt="License" src="https://img.shields.io/badge/License-MIT%2FApache--2.0-lightgrey"></a>
    </p>
</div>


Yew Styles is a style framework for [yew](https://yew.rs/docs/en/intro/) without JavaScript dependencies

## Motivation
The purpose of developing this project is first,
provide a style framework for yew that doesn't require any JavaScript dependencies
also to create a layout system which is not far of the flexbox concept, and,
to take the rust benefits and implement properties selected by enumeration
in the most of the cases which makes fast for developing applications and avoids the practice try and error

## How it works
Each component is split in two parts, the logical yew component and its sass module,
however, it is not necessary to worry about the sass module only it needs to be include in the project

**Note:** One of the goals of yew_styles project is include css in the components without depending of any external css file.
Possible candidates are [CssinRust](https://github.com/lukidoescode/css-in-rust) or [RUSS](https://github.com/siku2/russ)

### How install it
1. Install the sass module: `npm install yew-styles`
2. Add the yew_style crate with the features needed for your project in Cargo.toml file:
```toml
yew_styles = { version="0.11", features=["button", "text", "navbar"] }
```
3. Import the main.css file in you main javascript/typescript file project:
```typescript
    import 'node_modules/yew-styles/main.css';
```
4. Ready to import and use in your project 🚀

## Visual example:

You can find all the visual example in the website https://yewstyles.spielrs.tech

## Run the documentation page
1. `git clone https://github.com/spielrs/yew_styles.git`
2. `cd yew_styles`
3. `npm start`

In the left side there is a list of links where each one access to a correspondent component documentation,
there, shows how to use it.

## Rust Docs

You can also see all the Yew Style documentation in rust docs [here](https://docs.rs/crate/yew_styles).
It includes description and examples for each component

## Run the tests
Inside of the project run:

`cargo test --target wasm32-unknown-unknown --manifest-path=crate/yew_styles/Cargo.toml`

## Development phase
Yew Styles cover all the common cases used in a web application however there are still a lot of work to do and components to implement.
All contributions are appreciated.

## How contribute
First, open an issue describing about the fix, improvement or implementation and as suggestion, don't start to work in it until that is discussed.
If the contribution is a fix or small improvement in a component, only a pull request to master explaining what resolve or improve that, is required.
If it is an implementation, please follow the next requirements:

* Firstable open an issue describing about the component
* Unit tests, which checks that the component is created and
its logic works, in the same file where it is implemented (test events is not needed for now)
* One component per file, if multiple components have connections between them, it is possible create subfolder
* Documentation in the component showing an example of using it and small description of each prop
* Create a component page in `/crate/src/page` with the same structure than the rest of the components



## Do you like Yew Styles?
If you like Yew Styles, help me supporting the project:
- BAT rewards in case that you use [Brave Browser](https://brave.com/)
- Using this link to create an account in [Binance](https://www.binance.com/en/register?ref=DB8EPXF0) (get 10% fee back for every trading)

If you need a feature that is not cover yet, as soon as possible, you can also fund the issue [here](https://issuehunt.io/r/spielrs/yew_styles)

## Code of Conduct
Please check our [code of conduct](CODE_OF_CONDUCT.md)

## Code Contributors

<a href="https://github.com/dancespiele">
    <img src="https://github.com/dancespiele.png?size=50">
</a>
<a href="https://github.com/zoechi">
    <img src="https://github.com/zoechi.png?size=50">
</a>
<a href="https://github.com/ajstrand">
    <img src="https://github.com/ajstrand.png?size=50">
</a>
<a href="https://github.com/philip-peterson">
    <img src="https://github.com/philip-peterson.png?size=50">
</a>

## Roadmap

- [x] Button
- [x] Layout
- [x] Navbar
- [x] Form
- [x] Card
- [x] Text
- [x] Dropdown
- [ ] Progress
- [x] Spinners
- [ ] Table
- [ ] Pagination
- [x] Modal
- [ ] Sidebar
- [ ] Tab
- [x] Tooltips
- [ ] Collapse
- [x] Carousel
- [ ] Calendar
- [x] Assets (implemented in the new library [yew_assets](https://github.com/spielrs/yew_assets))

## License

Yew Style is [MIT](LICENSE-MIT.md) and [Apache-2.0](LICENSE-APACHE.md) licensed
