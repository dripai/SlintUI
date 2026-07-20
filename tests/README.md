# 测试与截图基线

```powershell
cargo fmt --all --check
cargo check --workspace --offline
cargo test --workspace --offline
```

自动交互 Harness 覆盖已实现组件的操作、导航、选择、日期时间边界、颜色、DataGrid 宿主窗口请求和 Calendar 选择，并验证禁用、重复与越界路径不会产生额外状态或回调。实现优先级只用于项目管理，不作为 Gallery 的浏览结构。

Gallery 有界运行：

```powershell
$env:SLINT_BACKEND='winit-software'
cargo run -p slint-ui-gallery --offline -- --smoke-test
```

生成单个组件基线（`--page` 选择一级分类，`--section` 选择二级组件）：

```powershell
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-general-button-100.png --page 0 --section 7 --theme light --density regular --scale 1.0 --locale zh-CN
```

代表性分类与图标目录基线：

```powershell
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-navigation-steps-rtl-100.png --page 6 --section 4 --theme light --density regular --scale 1.0 --locale ar-EG
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-data-entry-text-field-contrast-100.png --page 3 --section 4 --theme high-contrast --density regular --scale 1.0 --locale zh-CN
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-icons-outlined-100.png --page 5 --icon-style outlined --theme light --density regular --scale 1.0 --locale zh-CN
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-icons-filled-dark-100.png --page 5 --icon-style filled --theme dark --density compact --scale 1.0 --locale en-US
```

现有 8 张基线按“分类 + 单个组件”命名，覆盖七个一级分类、浅色/深色/高对比度、LTR/RTL、compact/regular、100%/150% 预览缩放，以及 outlined/filled 图标目录。125%/200% 与 comfortable 密度可在 Gallery 中交互检查，但不为每种组合保留固定图片。截图使用 `winit-software` 并强制减少动效，使动画相位不会造成无意义差异；跨系统字体差异仍需人工评审。

核心 crate 测试还会核对 447 个 outlined、249 个 filled SVG 与对应按需 Slint 模块一一存在，并防止 Checkbox、TextField 清除入口重新使用字体符号代替 SVG。

文件尺寸和 SHA-256 记录在 [`screenshots/manifest.json`](screenshots/manifest.json)。哈希用于确认同一 Windows 软件渲染环境内是否发生变化，不作为跨平台像素一致性的承诺。
