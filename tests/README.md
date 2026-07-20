# 测试与截图基线

```powershell
cargo fmt --all --check
cargo check --workspace --offline
cargo test --workspace --offline
```

自动交互 Harness 覆盖 P0 控件、里程碑 D 组件，以及 P2 的操作、导航、选择、日期时间边界、颜色、DataGrid 宿主窗口请求和 Calendar 选择，并验证禁用、重复与越界路径不会产生额外状态或回调。

Gallery 有界运行：

```powershell
$env:SLINT_BACKEND='winit-software'
cargo run -p slint-ui-gallery --offline -- --smoke-test
```

生成单张基线：

```powershell
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-light-foundations-100.png --page 0 --theme light --density regular --scale 1.0 --locale zh-CN
```

生成完整图标目录代表基线：

```powershell
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-light-icons-100.png --page 5 --icon-style outlined --theme light --density regular --scale 1.0 --locale zh-CN
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-dark-icons-filled-100.png --page 5 --icon-style filled --theme dark --density compact --scale 1.0 --locale en-US
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-light-desktop-p1-100.png --page 6 --theme light --density regular --scale 1.0 --locale zh-CN
cargo run -p slint-ui-gallery --offline -- --screenshot tests/screenshots/gallery-light-advanced-p2-100.png --page 7 --theme light --density regular --scale 1.0 --locale zh-CN
```

现有 9 张基线覆盖浅色、深色、高对比度、RTL、compact/regular/comfortable、100%/150% 预览缩放、outlined/filled 图标目录，以及里程碑 D/P2 代表组合。125%/200% 可在 Gallery 中交互检查，但不为每种组合保留固定图片。截图使用 `winit-software` 并强制减少动效，使动画相位不会造成无意义差异；跨系统字体差异仍需人工评审。

核心 crate 测试还会核对 447 个 outlined、249 个 filled SVG 与对应按需 Slint 模块一一存在，并防止 Checkbox、TextField 清除入口重新使用字体符号代替 SVG。

文件尺寸和 SHA-256 记录在 [`screenshots/manifest.json`](screenshots/manifest.json)。哈希用于确认同一 Windows 软件渲染环境内是否发生变化，不作为跨平台像素一致性的承诺。
