# P0 验收与发布状态

本文记录 `v0.1.0` P0 基线已经具备的能力、验证范围、豁免项和明确限制。设计目标仍以四份全局规范为准；未在此列为已验证的能力不得视为已经完成平台验收。

## 已实现范围

- 工作区：`slint-ui` 核心 crate、`slint-ui-gallery` 应用和 `@slint-ui/index.slint` 公开入口。
- 全局环境：浅色、深色、高对比度主题，三种密度，100%–200% 文本与预览缩放，Locale、LTR/RTL 和减少动效状态。
- 基础原语：Theme、Typography、Label、Icon、Surface、Divider、FocusRing。
- 图标资源：447 个 outlined、249 个 filled SVG，独立模块按需导入；Checkbox 和 TextField 状态图标不再依赖字体字符。
- 布局：Stack、Space、Card、ScrollArea。
- 操作：Button、IconButton、ToolButton、Toolbar。
- 输入：Checkbox、Switch、TextField、Select、SegmentedControl。
- 表单与反馈：FormRow、SettingsSection、Tooltip、Progress、Spinner、EmptyState、StatusBar。
- Gallery：基础、布局、操作、输入、反馈和图标页面；主题、密度、缩放、Locale、RTL 切换；软件渲染截图入口。

公开组件不添加 `Slint` 前缀；调用方通过 `@slint-ui/index.slint` 导入简短组件名。

## 原生能力与自定义边界

| 范围 | 实现方式 | 当前限制 |
|---|---|---|
| 文本输入 | Slint `TextInput` | 外观由 SlintUI 绘制；IME、选择、剪贴板和密码输入保留原生行为 |
| Select | Slint 标准 `ComboBox` 包装 | 无搜索；标准样式不能随运行时主题完全重绘 |
| ScrollArea | Slint 标准 `ScrollView` 包装 | 不提供虚拟化，超长数据集合应使用后续列表组件 |
| Tooltip | Slint 原生 `Tooltip` | 延迟和偏移不可配置，焦点触发提示尚未实现 |
| Button 系列 | `TouchArea`、`FocusScope` 与可访问属性组合 | 视觉状态统一由 Token 派生；Toolbar 内仍是顺序 Tab 访问 |
| Stack / Space | 稳定 `VerticalLayout` / `HorizontalLayout` | 不使用实验性 `FlexboxLayout`；方向不可运行时切换，Space 不换行 |
| RTL | 全局 direction、文本对齐和 Gallery Locale 场景 | 任意结构和方向图标不会自动镜像 |

## 验证矩阵

| 验证 | 当前覆盖 |
|---|---|
| 编译 | Windows、Rust 2024、Slint 1.17.1，workspace 全量检查 |
| 自动交互 | Button、IconButton、ToolButton、Checkbox、Switch、SegmentedControl、TextField 清除、Select 程序化选择；覆盖正常、禁用、重复和越界路径 |
| Gallery 冒烟 | 软件渲染后端创建并显示主窗口 |
| 截图 | 浅色基础、深色操作、高对比度输入、阿拉伯语 RTL 输入、150% 浅色反馈，以及浅色 outlined/深色 filled 图标目录，共 7 张基线 |
| 编码 | 仓库文本文件严格 UTF-8 解码和乱码特征扫描 |
| 人工视觉 | 当前 Windows 软件渲染截图检查 |
| 发布版本 | Cargo workspace `0.1.0`，固定 Git Tag `v0.1.0` |

## 尚未完成的平台验证

- Windows 原生 GPU 后端、系统 DPI、Narrator/NVDA 和多种输入法真机矩阵。
- macOS、Linux 的编译、字体、焦点、VoiceOver/Orca 与原生弹层检查。
- 125% 和 200% 的独立截图基线；Gallery 已能切换这些比例，但当前固定基线只保留代表性组合。

## P0 豁免项

- 首个真实业务设置页与工具栏接入：仓库内没有可迁移的外部业务项目，因此不虚构结果，也不以不可执行的外部条件阻塞 `v0.1.0`。首个采用方出现后必须补充独立接入记录。

P1、P2 和平台增强组件继续保持“待实现”，当前实现没有兼容层、备用行为或静默回退。以上平台限制不影响 P0 核心组件与当前 Windows 软件渲染基线完成，但不得被表述为三平台真机验收完成。
