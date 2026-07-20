# SlintUI

SlintUI 是一套基于 Slint 的独立桌面设计系统与组件库。它拥有自己的设计规范、组件源码、组件展示程序、测试和版本发布流程；具体业务项目只是它的使用者，不参与组件库内部实现。

当前阶段：P0 基线、里程碑 D 的核心桌面组件和里程碑 E 的全部 P2 核心组件已实现；其余 P1、真实业务项目接入和平台增强仍在规划中。

当前可直接运行：

```powershell
cargo run -p slint-ui-gallery
```

已实现的公开组件除 P0 外，还包括里程碑 D 的 Overlay、SplitPane、Tabs、菜单、Table、Tree、Form、Toast 与 Dialog，以及里程碑 E 的 20 个 P2 组件。逐项能力与限制见 [`docs/p1-status.md`](docs/p1-status.md) 和 [`docs/p2-status.md`](docs/p2-status.md)。

图标库已提供 447 个 outlined 和 249 个 filled SVG。图标通过独立 Slint 模块按需导入，不在主 `index.slint` 中聚合，避免普通组件使用者无条件携带整套资源。用法与完整清单见 [`docs/iconography.md`](docs/iconography.md)。

## 1. 项目目标

SlintUI 的目标是提供一套稳定、统一、可复用的桌面 UI 基础设施，使不同产品不再重复设计和实现按钮、输入框、工具栏、设置分组等通用界面。

组件库负责：

- 颜色、字体、间距、圆角、阴影和动画等视觉规范。
- 默认、悬停、按下、选中、禁用和聚焦等通用状态。
- 键盘操作、焦点管理和可访问语义。
- 组件属性、回调、尺寸和组合规则。
- 深色、浅色和高对比度主题。
- 不同系统缩放比例下的一致表现。

组件库不负责：

- 截图、录屏、OCR、文件处理等业务能力。
- 产品状态机、数据持久化和网络请求。
- Windows、Linux 或 macOS 平台业务 API。
- 某个产品专用的工具名称、文案和流程。

例如，组件库可以提供 `ToolButton`，但不提供 `ScreenshotButton` 或 `StartRecordingButton`。业务项目通过属性和回调组合通用组件。

## 2. 核心原则

### 2.1 先设计，后实现

每个组件必须先确定视觉规范、尺寸、状态和交互，再编写 Slint 代码。不能一边实现业务页面，一边临时决定公共组件样式。

### 2.2 优先使用 Slint 原生行为

实现顺序为：

1. Slint 原生组件和平台能力。
2. 对原生组件进行外观包装和组合。
3. 使用少量自定义组件补足视觉结构。
4. 只有原生公开能力无法满足要求时，才自行实现行为层。

菜单、弹层、输入、焦点和键盘操作应尽量交给 Slint 原生组件处理，避免重复实现已经存在的关闭策略、定位、可访问性和跨平台行为。

### 2.3 稳定接口与视觉实现分离

业务项目只依赖组件公开的属性、回调、枚举和结构体，不依赖组件内部元素。组件库可以调整内部绘制方式，但不能在补丁版本中破坏公开接口。

### 2.4 通用组件与产品组件分离

可以跨两个以上产品复用的能力才进入 SlintUI。带有明确业务语义的组合留在对应产品中。

### 2.5 Gallery 即组件文档

组件没有进入 Gallery、没有覆盖完整状态、没有 API 说明，就不算完成。

### 2.6 核心组件与平台增强严格分层

SlintUI 定义两个兼容层级：

- 核心组件层：只包含纯 `.slint` 组件，不调用 Windows、macOS 或 Linux 系统 API，目标是在三个桌面平台复用同一套组件源码。
- 平台增强层：窗口拖动、原生阴影、托盘、全局快捷键和其他系统集成分别提供 Windows、macOS、Linux 实现。

核心组件不能依赖任何平台增强包。平台增强包可以依赖核心组件和统一的平台能力接口。业务项目按目标平台选择增强实现，不需要平台能力时可以只依赖核心组件。

平台差异不能通过在核心组件中不断堆积系统判断解决。确实需要不同实现的能力必须进入对应平台包，并保持一致的公开语义。

## 3. 仓库结构

```text
SlintUI/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── crates/
│   └── slint-ui/
│       ├── src/lib.rs             # @slint-ui library path helper
│       ├── ui/index.slint          # public exports
│       ├── ui/tokens/
│       ├── ui/primitives/
│       ├── ui/controls/
│       ├── ui/layout/
│       ├── ui/feedback/
│       ├── ui/icons/
│       └── assets/icons/ant-design/
├── apps/
│   └── gallery/
│       ├── src/main.rs
│       └── ui/gallery.slint
├── design/
│   ├── *.md
│   └── component-specs/*.md
├── docs/
│   ├── component-api/
│   ├── iconography.md
│   └── p0-status.md
├── tools/icons/
├── THIRD_PARTY_NOTICES.md
└── tests/
    └── screenshots/*.png
```

平台能力接口及 Windows、macOS、Linux 增强 crate 尚未实现，不能按上方分层原则提前从核心组件调用系统 API。

## 4. 组件分层

### 4.1 Tokens

Tokens 是所有视觉决策的唯一来源：

- 语义颜色，而不是页面中的临时色值。
- 字体、字号、字重和行高。
- 4px 或其他统一间距体系。
- 圆角、边框、阴影和动画时长。
- 控件的小、中、大尺寸。

示例：

```slint
export global Theme {
    property <color> background: #f5f5f5;
    property <color> surface: #ffffff;
    property <color> primary: #1677ff;
    property <color> success: #52c41a;
    property <color> warning: #faad14;
    property <color> error: #ff4d4f;
    property <color> text: #000000e0;
    property <color> muted-text: #000000a6;

    property <length> radius-small: 4px;
    property <length> radius-medium: 6px;
    property <length> control-height-small: 24px;
    property <length> control-height-medium: 32px;
    property <length> control-height-large: 40px;
}
```

以上数值是首期视觉基线，不允许组件直接复制色值。完整的浅色、深色、高对比度、排版、间距、圆角、阴影和动效规范见 [`design/foundations.md`](design/foundations.md)。

Slint 的 `global` 在不同顶层窗口之间不会自动共享同一个实例。动态主题需要由宿主业务代码为每个窗口同步，不能假设修改一个窗口会自动更新其他窗口。

### 4.2 Primitives

Primitives 是没有业务含义的最小视觉单元，例如 Surface、Divider、Icon 和 FocusRing。它们不直接承载复杂交互。

`Icon` 负责尺寸、Token 着色和可访问名称；具体 SVG 通过 `@slint-ui/icons/<style>/<name>.slint` 按需导入。Outlined 是默认系统操作风格，Filled 仅用于选中、强状态或品牌图形。

### 4.3 Controls

Controls 是用户直接操作的控件，例如按钮、开关、输入框和选择器。它们必须公开稳定、类型明确的接口。

```slint
export enum ButtonVariant {
    normal,
    primary,
    danger,
    ghost,
}

export component ToolButton {
    in property <string> text;
    in property <image> icon;
    in property <ButtonVariant> variant: normal;
    in property <bool> checked: false;
    in property <bool> enabled: true;

    callback clicked();
}
```

优先使用枚举表达有限状态，不用任意字符串传递组件变体。

### 4.4 Layout

Layout 组件处理页面结构和一致间距，例如卡片、表单行和设置分组。它们不处理业务数据。

### 4.5 Feedback

Feedback 组件负责状态、进度、空状态和提示。对话框、菜单和弹层应优先包装 Slint 原生能力，不创建不必要的自定义窗口管理逻辑。

## 5. 组件接口规范

每个组件必须明确：

- 组件用途与不适用场景。
- 属性、类型和默认值。
- 回调及其触发时机。
- 可用尺寸和视觉变体。
- 默认、悬停、按下、焦点、选中、禁用和加载状态。
- 键盘行为和可访问语义。
- 中文、英文和超长文本的处理策略。
- 最小尺寸、最大尺寸和布局约束。
- 是否支持深色、浅色和高对比度主题。

禁止业务项目访问组件内部命名元素。需要公开的能力必须通过属性、回调或公开函数提供。

## 6. Gallery 设计

`apps/gallery` 是独立可运行的组件展示程序，类似成熟后台组件库的文档站。

每个组件页面至少展示：

- 所有视觉变体。
- 所有尺寸。
- 默认、悬停、按下、焦点、禁用和加载状态。
- 选中与未选中状态。
- 中文、英文、空文字和超长文字。
- 深色、浅色和高对比度主题。
- 100%、125%、150% 和 200% 系统缩放。
- 键盘操作说明。
- 属性、回调、枚举和默认值。
- 推荐用法和错误用法。

Gallery 同时承担人工验收、截图基线生成和设计评审功能。组件修改后必须先在 Gallery 中验证，再发布新版本。

## 7. 视觉基线与组件规划

SlintUI 参考 Ant Design 成熟的企业级信息层级、色彩语义、Token 分层和组件分类，但不依赖 Ant Design 代码，也不要求任何外部设计工具。桌面端交互、尺寸、窗口行为和公开 API 仍以 Slint 能力与本项目需求为准。

- [`design/foundations.md`](design/foundations.md)：定义 Token 派生、颜色、排版、密度、尺寸、窗口适配、层级、动效和主题环境。
- [`design/interaction.md`](design/interaction.md)：定义状态模型、事件、键盘、焦点、浮层、异步反馈和导航协议。
- [`design/accessibility.md`](design/accessibility.md)：定义对比度、键盘、语义、命中区域、文本缩放和平台无障碍验收标准。
- [`design/content-and-localization.md`](design/content-and-localization.md)：定义文案、数据格式、Locale、RTL、时间、空值和隐私规则。
- [`design/component-inventory.md`](design/component-inventory.md)：定义完整组件清单、分层、优先级、目标文件和完成标准。
- `design/component-specs/`：记录每个已实现组件的用途、API、状态、键盘行为、可访问语义、原生能力与限制。

所有实现使用同一套公开命名：组件文档中的 Variant、State、Size 必须与 Slint 枚举和属性一致，禁止设计说明与代码各自维护一套概念。

## 8. 独立发布与接入

Slint 官方支持通过 library path 引用外部组件库。SlintUI 使用一个轻量 Rust crate 负责暴露组件目录，业务项目通过 Git tag 或 crates.io 固定版本接入。

业务项目的 `Cargo.toml`：

```toml
[build-dependencies]
slint-ui = { git = "https://github.com/example/slint-ui.git", tag = "v0.1.0" }
slint-build = "=1.17.1"
```

业务项目的 `build.rs`：

```rust
fn main() {
    let paths = std::collections::HashMap::from([
        ("slint-ui".to_owned(), slint_ui::slint_library_path()),
    ]);

    let config =
        slint_build::CompilerConfiguration::new().with_library_paths(paths);

    slint_build::compile_with_config("ui/app.slint", config)
        .expect("compile application UI");
}
```

Slint 文件中使用：

```slint
import {
    Button,
    ToolButton,
    Switch,
    Theme,
} from "@slint-ui/index.slint";
```

业务项目不能复制组件源码。发现问题时，应在 SlintUI 仓库修复、验证并发布新版本，再由业务项目升级依赖。

## 9. 版本与兼容策略

使用语义化版本：

- `0.1.x`：设计探索阶段，接口可能调整。
- `0.5.x`：主要组件和主题机制基本稳定。
- `1.0.0`：公开接口、设计变量和发布流程稳定。
- 只修复样式且不改变接口：补丁版本。
- 新增向后兼容的组件或能力：次版本。
- 删除属性、枚举或改变既有行为：主版本。

初始兼容基线：

```text
SlintUI 0.1.x
Slint 1.17.1
Rust 2024 Edition
核心组件目标：Windows、macOS、Linux
首个完整验证平台：Windows 10/11
Linux、macOS：CI 编译与后续真机验证
```

首个固定 P0 基线为 Git Tag `v0.1.0`；组件能力、验证范围和豁免项见 [`docs/p0-status.md`](docs/p0-status.md)。

三个桌面平台必须分别验证渲染、字体、焦点、菜单、缩放和平台行为。后续扩展 Android、Web 或嵌入式时新增独立兼容基线，不能降低桌面组件的交互密度和质量。

## 10. 质量门禁

一个组件只有满足以下条件才能发布：

- 设计规范已评审。
- 公开接口已记录。
- 使用 Slint 原生组件的可行性已经核对。
- 必须自定义时已记录原生能力的具体限制。
- Gallery 覆盖完整状态与尺寸。
- 键盘和焦点行为可用。
- 通过 UTF-8、格式和编译检查。
- 通过 100%、125%、150% 和 200% 缩放检查。
- 深色与浅色主题没有明显对比度问题。
- 截图基线已更新并通过人工评审。
- 没有引入具体产品的业务概念。

“编译通过”只表示代码可构建，不代表视觉、交互和当前设备行为已经验证。

## 11. 实施路线

### 阶段一：基础设计（已完成）

1. 确定项目名称、许可和版本策略。
2. 完成颜色、字体、间距、圆角、阴影和动画规范。
3. 完成视觉基础规范和组件命名规则。
4. 建立 `Theme` 与基础 Tokens。

### 阶段二：Gallery（已完成 P0 基线）

1. 创建独立 Gallery 应用。
2. 实现组件导航、主题切换和缩放预览。
3. 建立组件状态展示模板。
4. 建立截图基线目录和评审流程。

### 阶段三：首批组件（已完成）

按以下顺序实现：

1. Button、IconButton、ToolButton 和 Toolbar。
2. Switch、Checkbox、TextField、Select 和 SegmentedControl。
3. Stack、Space、Card、FormRow 和 SettingsSection。
4. Tooltip、Progress、Spinner、EmptyState 和 StatusBar。

具体范围与完成标准以 [`design/component-inventory.md`](design/component-inventory.md) 的 P0 清单为准。首批不追求组件数量，优先保证设计一致、状态完整和接口稳定。

### 阶段四：首个产品验证（待实现）

1. 发布 `v0.1.0`。
2. 由首个业务项目通过固定 Git tag 接入。
3. 优先迁移设置页，验证表单类组件。
4. 再迁移工具栏，验证高频交互组件。
5. 组件问题回到 SlintUI 修复，禁止在业务项目中复制后修改。

### 阶段五：稳定发布（待实现）

1. 根据首个产品反馈收敛接口。
2. 补全主题、可访问性和缩放测试。
3. 建立变更日志和升级说明。
4. 接入第二个业务项目，验证真正的跨产品复用。
5. 达到稳定标准后发布 `1.0.0`。

## 12. 当前已知限制

- Slint 1.17.1 的稳定布局不能让一个公开容器在运行时切换其 `@children` 方向，也不提供稳定的自动换行布局；因此 `Stack` 固定为垂直排列，`Space` 固定为水平排列且不自动换行。
- `Select` 复用标准 `ComboBox` 以保留原生弹层、键盘和焦点行为。标准控件样式在编译期选择，不能随 SlintUI 的运行时主题完全重绘；搜索型选择器不属于当前 P0。
- Slint 1.17.1 的原生 Tooltip 不公开延迟和偏移参数；当前实现使用原生悬停定位，焦点触发提示仍待上游公开能力或后续经过评审的自定义方案。
- `Toolbar` 支持顺序 Tab 访问；跨任意 `@children` 实现 roving focus、方向键遍历和自动溢出需要稳定的子项接口，当前未实现。
- RTL 环境、阿拉伯语文案与文本对齐已实现；任意业务组合的结构镜像、方向图标镜像和视觉方向键顺序仍需调用方显式编排，不能宣称自动覆盖。
- 组件已声明 Slint 可访问角色、名称、状态和值，并通过键盘与 Gallery 检查；Windows Narrator、NVDA、VoiceOver 和 Orca 的真机读取结果尚未形成自动化门禁。
- `ScrollArea` 复用标准 `ScrollView`，会实例化其全部子项，不适用于需要虚拟化的超长列表。
- Gallery 的缩放是设计预览环境；Windows、macOS、Linux 的真实 DPI、字体、输入法与屏幕阅读器仍需分别验证。

完整实现状态和验证矩阵见 [`docs/p0-status.md`](docs/p0-status.md)。

## 13. 首期完成标准

首期不是以组件数量判断完成，而是满足以下结果：

- 独立仓库可自行编译和运行。
- Gallery 可以脱离任何业务项目展示全部首批组件。
- 设计变量不存在散落在组件中的重复常量。
- 首批组件的接口、状态和文档完整。
- 业务项目可以通过固定版本依赖接入。
- 业务项目不需要复制或修改 SlintUI 源码。
- 至少一个真实设置页完成迁移验证。

## 14. 官方能力依据

- Slint 组件、模块与组件库路径：<https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/>
- Slint 全局单例：<https://docs.slint.dev/latest/docs/slint/guide/language/coding/globals/>
- Slint 标准组件与样式：<https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/>
- Slint Viewer：<https://docs.slint.dev/latest/docs/slint/guide/tooling/slint-viewer/>
- Ant Design 设计价值观：<https://ant.design/docs/spec/values/>
- Ant Design 色彩规范：<https://ant.design/docs/spec/colors/>
- Ant Design 字体规范：<https://ant.design/docs/spec/font/>
- Ant Design 布局与间距：<https://ant.design/docs/spec/layout/>、<https://ant.design/docs/spec/proximity/>
- Ant Design 主题与 Design Token：<https://ant.design/docs/react/customize-theme/>
- Ant Design 组件总览：<https://ant.design/components/overview/>
