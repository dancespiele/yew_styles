# `yewstyle`

This is an initial project of a framework style for yew

you can run the project with:

* `npm start`

To install yew-styles in your project, execute in the root path:

* `npm install yew-styles`

* after import the css in your index.js/index.ts file:

```javascript
import 'node_modules/yew-styles/main.css';
```

* include in your Cargo.toml if you want to use web-sys feature this:

```toml
yew_styles= {path= "./yew_styles", version = "0.2.0", features = ["web_sys"]}
```

or this if you want to use stdweb instead

```toml
yew_styles= {path= "./yew_styles", version = "0.2.0", features = ["std_web"]}
```