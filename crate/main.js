import init, { run_app } from './pkg/yewstyle_page.js';
async function main() {
   await init('/pkg/yewstyle_page_bg.wasm');
   run_app();
}
main()