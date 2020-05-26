const { execSync } = require('child_process');
const Bundler = require('parcel-bundler');
const Path = require('path');
const chokidar = require('chokidar');
const dotenv = require('dotenv');
dotenv.config();

const entryFiles = Path.join(__dirname, 'index.html');

const buildType = process.argv[2];

const options = {
    outDir: './dist', 
    outFile: 'index.html',
    publicUrl: '/',
    watch: true,
    minify: buildType === 'production',
  };

(async () => {
    const bundler = new Bundler(entryFiles, options);

    chokidar.watch(['./crate/src', './crate/Cargo.toml', './crate/yew_styles/src', './crate/yew_styles/Cargo.toml']).on('change', async (event, path) => {
        console.log(`there are new changes in '${path}'. Start to rebuild rustwasm sources`);

        bundler.bundle();

        bundler.hmr.broadcast({
            type: 'reload'
        });
    });

    bundler.on('buildStart', () => {
        const prevtBuildFile = Path.join(__dirname, './wasm_pack_cmd');
        console.log(`running: ${prevtBuildFile}`);
        execSync(`${prevtBuildFile} ${buildType === 'production' ? '' : '--dev'}`, {stdio: 'inherit'});
    });

    await bundler.serve(process.env.SERVER_ADDRESS || 1234);
})();

