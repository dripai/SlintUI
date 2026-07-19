# 接入 SlintUI

## 运行 Gallery

```powershell
cargo run -p slint-ui-gallery
```

有界启动验证：

```powershell
$env:SLINT_BACKEND='winit-software'
cargo run -p slint-ui-gallery -- --smoke-test
```

## 在 Rust 项目中注册组件库

```toml
[build-dependencies]
slint-ui = { path = "../SlintUI/crates/slint-ui" }
slint-build = "=1.17.1"
```

```rust
let paths = std::collections::HashMap::from([
    ("slint-ui".to_owned(), slint_ui::slint_library_path()),
]);
let config = slint_build::CompilerConfiguration::new().with_library_paths(paths);
slint_build::compile_with_config("ui/app.slint", config)?;
```

```slint
import { Button, TextField, Theme } from "@slint-ui/index.slint";
```

组件源码不复制到业务仓库；修复后升级组件库版本。

