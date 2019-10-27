declare module "*.toml" {
    const module: {run: () => void};
    export default module;
}