import './yew-styles/main.sass';
import './page-styles/main.sass';
import 'prismjs/themes/prism.css';
import 'prismjs/components/prism-rust';

import module from '../crate/Cargo.toml';
module.run();
