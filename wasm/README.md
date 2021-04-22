# yukicoder-md-wasm

# Build

```bash
$ wasm-pack build --release
```

output to wasm/pkg/

# Import NPM module

```bash
$ yarn link path/to/wasm/pkg
```

npm project

```typescript
import {convert} from 'yukicoder_md_wasm';

let html_string = convert(markdown_string, enable_template_engine);
```
