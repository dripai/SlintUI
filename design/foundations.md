# SlintUI 视觉基础规范

本文定义 SlintUI 首期可执行的视觉基线。它参考 Ant Design 的企业级界面设计语言和 Design Token 方法，但所有名称、数值、组件接口和实现都属于 SlintUI，不依赖 Ant Design 运行时代码或外部设计工具。

## 1. 设计方向

SlintUI 面向高信息密度的桌面应用，视觉目标是清晰、克制、稳定和高效：

- 自然：视觉层级与反馈符合用户预期，不用装饰掩盖功能。
- 确定：相同操作在不同页面使用相同组件、状态和反馈。
- 有意义：颜色、强调、动效都必须传递信息或帮助完成任务。
- 可成长：Token、组件和组合模式允许产品扩展，但不破坏既有语义。

参考不是复制。网页特有的响应式布局、悬浮按钮和营销型展示不直接进入核心组件；窗口、焦点、键盘、右键菜单和高 DPI 是桌面端的优先事项。

## 2. Token 分层

所有样式必须通过三层 Token 表达，不在组件内散落常量。

1. 基础 Token：品牌色、功能色、字号基线、圆角基线、控件高度等设计输入。
2. 语义 Token：文本、背景、边框、交互状态、阴影和焦点环等实际用途。
3. 组件 Token：只在单个组件确有特殊需要时定义，例如 Button 的水平内边距或 Table 的行高。

优先修改基础或语义 Token。组件 Token 不得重新定义已有语义，也不得反向影响其他组件。

### 2.1 派生规则

SlintUI 使用“基础输入 → 主题映射 → 语义别名 → 组件覆盖”的单向派生流程：

```text
品牌色、功能色、字号、基础圆角、基础尺寸
                    ↓
浅色 / 深色 / 高对比度 / 密度模式映射
                    ↓
文本、背景、边框、状态、层级等语义 Token
                    ↓
必要且局部的组件 Token
```

- 基础 Token 不能被组件直接消费。
- 每个主题必须提供同名、同类型的完整语义 Token，禁止组件根据主题名称分支。
- 主题映射可以由构建脚本生成，但发布包中的最终值必须固定、可审查并进入截图基线。
- 功能色不能只提供一个基准色，必须同时提供背景、边框、文本、Hover、Pressed 和焦点轮廓。
- 局部主题只能覆盖语义 Token 或组件 Token，未覆盖项继承外层主题；局部覆盖不得改变组件行为。
- 新增 Token 前先证明现有语义无法表达该用途，并记录所有消费方。

### 2.2 命名与版本治理

- 项目名保留 `SlintUI`，组件库路径使用 `@slint-ui`；公开组件直接使用 `Button`、`TextField`、`Theme` 等语义名称，不添加 `Slint` 前缀。
- 与标准组件或其他库重名时，在 `import` 处使用别名，例如 `import { Button as StandardButton } from "std-widgets.slint";`，不把依赖来源固化进公开组件名。
- 不占用 Slint 内置元素名称。需要封装 `Image`、`Dialog`、`Menu`、`MenuItem` 时，公开名称分别使用 `ImageView`、`ModalDialog`、`PopupMenu`、`PopupMenuItem`。
- 名称按“类别-用途-状态-尺寸”组织，例如 `color-primary-bg-hover`、`control-height-small`。
- Token 名称表达用途，不包含页面、产品或组件内部元素名称。
- 重命名、删除或改变既有 Token 语义属于破坏性变更；只调整色值但不改变用途属于视觉变更，必须更新截图基线。
- 组件 Token 使用组件前缀，例如 `button-padding-horizontal`，不得覆盖全局 Token 的含义。
- 文档、Slint 属性、Gallery 标签和测试基线使用同一名称。

建议文件：

```text
ui/tokens/
├── colors.slint
├── typography.slint
├── spacing.slint
├── size.slint
├── radius.slint
├── border.slint
├── elevation.slint
└── motion.slint
```

## 3. 色彩系统

### 3.1 品牌色阶

主色使用蓝色，默认操作色为 `#1677FF`。完整色阶用于生成悬停、按下、浅背景和强调层，不允许组件自行调亮或调暗。

| Token | 色值 | 用途 |
|---|---:|---|
| `blue-1` | `#E6F4FF` | 选中浅背景、信息提示背景 |
| `blue-2` | `#BAE0FF` | 浅色边框 |
| `blue-3` | `#91CAFF` | 弱强调边框 |
| `blue-4` | `#69B1FF` | 浅色主题强调边框 |
| `blue-5` | `#4096FF` | 主按钮悬停 |
| `blue-6` | `#1677FF` | 主色、链接、焦点 |
| `blue-7` | `#0958D9` | 主按钮按下 |
| `blue-8` | `#003EB3` | 强调深色 |
| `blue-9` | `#002C8C` | 深色辅助 |
| `blue-10` | `#001D66` | 最深品牌色 |

### 3.2 功能色

| 语义 Token | 基准色 | 使用边界 |
|---|---:|---|
| `color-primary` / `color-info` | `#1677FF` | 主要操作、链接、信息状态 |
| `color-success` | `#52C41A` | 已完成、可用、校验通过 |
| `color-warning` | `#FAAD14` | 风险、需要注意、可继续 |
| `color-error` | `#FF4D4F` | 失败、危险操作、校验错误 |

功能色必须同时配合图标或文字，不能只依靠颜色表达状态。危险按钮只用于不可逆或高风险操作，普通取消操作不得使用红色。

### 3.3 功能色语义矩阵

浅色主题使用以下首期固定值。后续如采用生成工具，生成结果必须与这些语义角色逐项对应。

| 语义 | Primary / Info | Success | Warning | Error |
|---|---:|---:|---:|---:|
| `*-bg` | `#E6F4FF` | `#F6FFED` | `#FFFBE6` | `#FFF2F0` |
| `*-bg-hover` | `#BAE0FF` | `#D9F7BE` | `#FFF1B8` | `#FFF1F0` |
| `*-border` | `#91CAFF` | `#B7EB8F` | `#FFE58F` | `#FFCCC7` |
| `*-border-hover` | `#69B1FF` | `#95DE64` | `#FFD666` | `#FFA39E` |
| `*-hover` | `#4096FF` | `#73D13D` | `#FFC53D` | `#FF7875` |
| `*-base` | `#1677FF` | `#52C41A` | `#FAAD14` | `#FF4D4F` |
| `*-active` | `#0958D9` | `#389E0D` | `#D48806` | `#D9363E` |

深色主题不能复用浅色状态色。首期使用独立基准与低明度背景：

| 语义 | Primary / Info | Success | Warning | Error |
|---|---:|---:|---:|---:|
| `*-bg` | `#111A2C` | `#162312` | `#2B2111` | `#2C1618` |
| `*-border` | `#15325B` | `#274916` | `#594214` | `#5B2526` |
| `*-hover` | `#3C89E8` | `#6ABE39` | `#E8B339` | `#E86E6B` |
| `*-base` | `#1668DC` | `#49AA19` | `#D89614` | `#DC4446` |
| `*-active` | `#1554AD` | `#3C8618` | `#AA7714` | `#AD393A` |

功能色文本是否使用 `*-base`、`*-hover` 或反色文本，必须由语义 Token 决定，组件不得直接引用色阶编号。

### 3.4 浅色主题

| 语义 Token | 色值 |
|---|---:|
| `bg-layout` | `#F5F5F5` |
| `bg-surface` | `#FFFFFF` |
| `bg-elevated` | `#FFFFFF` |
| `bg-control` | `#FFFFFF` |
| `text-primary` | `#000000E0` |
| `text-secondary` | `#000000A6` |
| `text-tertiary` | `#00000073` |
| `text-disabled` | `#00000040` |
| `text-placeholder` | `#00000040` |
| `text-inverse` | `#FFFFFF` |
| `text-on-accent` | `#FFFFFF` |
| `icon-default` | `#00000073` |
| `icon-hover` | `#000000E0` |
| `border-default` | `#D9D9D9` |
| `border-secondary` | `#0505050F` |
| `fill-hover` | `#0000000F` |
| `fill-pressed` | `#0000001A` |
| `bg-disabled` | `#0000000A` |
| `selection-bg` | `#E6F4FF` |
| `focus-outline` | `#1677FF` |
| `overlay-mask` | `#00000073` |

### 3.5 深色主题

| 语义 Token | 色值 |
|---|---:|
| `bg-layout` | `#000000` |
| `bg-surface` | `#141414` |
| `bg-elevated` | `#1F1F1F` |
| `bg-control` | `#141414` |
| `text-primary` | `#FFFFFFD9` |
| `text-secondary` | `#FFFFFFA6` |
| `text-tertiary` | `#FFFFFF73` |
| `text-disabled` | `#FFFFFF40` |
| `text-placeholder` | `#FFFFFF40` |
| `text-inverse` | `#000000E0` |
| `text-on-accent` | `#FFFFFF` |
| `icon-default` | `#FFFFFF73` |
| `icon-hover` | `#FFFFFFD9` |
| `border-default` | `#424242` |
| `border-secondary` | `#FDFDFD1F` |
| `fill-hover` | `#FFFFFF14` |
| `fill-pressed` | `#FFFFFF1F` |
| `bg-disabled` | `#FFFFFF0A` |
| `selection-bg` | `#111A2C` |
| `focus-outline` | `#3C89E8` |
| `overlay-mask` | `#000000A6` |

深色主题不是浅色主题的简单反相。阴影、边框、填充和功能色需要分别验收；浮层必须比承载它的表面更高一级。

### 3.6 高对比度主题

高对比度主题使用独立语义 Token。当前 P0 软件渲染基线固定为：

| 语义 Token | 色值 |
|---|---:|
| `bg-layout` / `bg-surface` / `bg-elevated` | `#000000` |
| `bg-disabled` | `#202020` |
| `text-primary` / `text-secondary` / `text-tertiary` | `#FFFFFF` |
| `text-disabled` / `text-placeholder` | `#A0A0A0` |
| `text-inverse` | `#000000E0` |
| `text-on-accent` | `#000000` |
| `border-default` / `border-secondary` | `#FFFFFF` |
| `fill-hover` / `fill-pressed` | `#FFFFFF26` / `#FFFFFF40` |
| `selection-bg` | `#FFFFFF` |
| `color-primary` / `color-primary-active` | `#00FFFF` |
| `color-primary-hover` | `#FFFFFF` |
| `color-primary-bg` / `color-primary-border` | `#000000` / `#00FFFF` |
| `color-success` / `color-warning` / `color-error` | `#00FF00` / `#FFFF00` / `#FF8080` |
| `color-error-hover` / `color-error-active` | `#FFFFFF` / `#FF8080` |
| `focus-outline` | `#FFFF00` |
| `overlay-mask` | `#00000073` |

实施与平台映射遵循以下约束：

- 正文和背景至少达到 WCAG AA；正文、标题等核心文本以 AAA 为目标。
- 焦点环、选中态和错误态必须有非颜色差异。
- 禁用态仍需可辨认，但不能被误认为可操作。
- 系统提供高对比度颜色时，平台增强层优先跟随系统语义色。

## 4. 排版

优先使用系统界面字体，避免捆绑字体造成跨平台字形和许可问题。

```text
Windows: Segoe UI, Microsoft YaHei UI
macOS:   SF Pro / system-ui, PingFang SC
Linux:   Noto Sans, Noto Sans CJK SC
Code:    Cascadia Mono, SFMono-Regular, Consolas, monospace
```

| Token | 字号 / 行高 | 字重 | 用途 |
|---|---:|---:|---|
| `caption` | `12 / 20px` | 400 | 辅助说明、快捷键提示 |
| `body` | `14 / 22px` | 400 | 默认正文和控件文字 |
| `label` | `14 / 22px` | 500 | 表单标签、强调文本 |
| `title-small` | `16 / 24px` | 500 | 卡片、分组标题 |
| `title-medium` | `20 / 28px` | 500 | 页面标题 |
| `title-large` | `24 / 32px` | 600 | 少量一级标题 |

单个产品通常只使用 3 至 5 个字阶。数字列表、统计值和表格数值使用等宽数字；正文不使用 600 以上字重制造层级。

## 5. 间距与布局

基础原子单位为 4px，主要布局节奏使用 8px 倍数。

| Token | 数值 | 常见用途 |
|---|---:|---|
| `space-1` | `4px` | 图标微调、紧凑内部间距 |
| `space-2` | `8px` | 同组元素、图标与文字 |
| `space-3` | `12px` | 控件内边距、紧凑列表 |
| `space-4` | `16px` | 表单行、卡片内边距 |
| `space-6` | `24px` | 分组间距、普通页面留白 |
| `space-8` | `32px` | 大区块间距 |
| `space-12` | `48px` | 空状态和宽松页面区块 |

- 同一信息组内部优先使用 8px，普通分组使用 16px，大分组使用 24px。
- 桌面布局优先支持动态窗口尺寸和最小尺寸，不照搬固定网页画板。
- 表单标签对齐、表格列宽、工具栏密度必须在组件层统一，页面不得逐项微调。
- 可拉伸区域、固定区域和滚动区域必须在组件规格中明确。

### 5.1 逻辑像素与系统缩放

- 本文中的 `px` 表示 Slint 逻辑像素，业务代码不得手工乘以系统缩放系数。
- 边框、焦点环和矢量图标使用逻辑尺寸；位图资源必须提供足够分辨率，禁止依赖插值修复模糊。
- 100%、125%、150%、200% 缩放下，布局尺寸、点击区域和文本不能被裁剪；截图基线分别维护。
- 文本缩放与界面缩放是两个独立输入。文本放大后优先增加控件高度或换行，不能截断关键操作。
- 窗口跨显示器移动后由宿主重新同步缩放、主题和高对比度状态。

### 5.2 窗口尺寸等级

尺寸等级按当前窗口内容区宽度计算，不按物理屏幕分辨率计算：

| 等级 | 内容区宽度 | 基本策略 |
|---|---:|---|
| `narrow` | `< 720px` | 表单标签置顶、侧栏折叠、操作允许换行 |
| `standard` | `720px–1199px` | 默认双列或侧栏布局 |
| `wide` | `>= 1200px` | 限制正文最大宽度，额外空间分配给数据区 |

- 通用窗口建议最小内容区为 `480 × 320px`；具体产品可以提高但不能降低组件最小尺寸。
- 布局切换必须保持焦点、滚动位置和业务状态，不通过重建页面实现。
- 设置表单的可读内容宽度建议不超过 `960px`；数据表和时间轴可以使用完整宽度。
- SplitPane、侧栏和工具区必须定义最小、首选、最大尺寸以及窗口恢复时的裁剪规则。

## 6. 尺寸、圆角、边框与阴影

### 6.1 全局密度

密度是整个界面的环境属性，Size 是单个组件的尺寸变体，两者不能混为一个枚举。

| 密度 | Small | Medium（默认） | Large | 默认行高 | 水平内边距 |
|---|---:|---:|---:|---:|---:|
| `compact` | `24px` | `28px` | `36px` | `32px` | `8px` |
| `regular` | `24px` | `32px` | `40px` | `40px` | `12px` |
| `comfortable` | `32px` | `40px` | `48px` | `48px` | `16px` |

- 默认密度为 `regular`。高密度表格和专业工具可以由宿主整体切换为 `compact`，页面不能只压缩个别组件。
- `comfortable` 用于触摸、文本放大或用户明确选择的宽松模式，实际命中区域至少 `44px`。
- 密度切换只能影响尺寸、间距和布局，不能改变数据、交互语义或可用功能。
- Overlay、菜单、Tooltip 和 Dialog 继承打开它们的窗口密度；系统级通知使用平台密度。

### 6.2 控件尺寸

| Size | 高度 | 用途 |
|---|---:|---|
| `small` | `24px` | 高密度工具栏、表格内操作 |
| `medium` | `32px` | 默认按钮、输入和选择控件 |
| `large` | `40px` | 低密度表单和关键操作 |

24px 控件只用于鼠标密集场景；触摸或可访问性模式应使用 40px 或更大的命中区域。图标标准尺寸为 12、16、20、24px，默认控件图标为 16px。

### 6.3 圆角与边框

| Token | 数值 | 用途 |
|---|---:|---|
| `radius-small` | `4px` | 标签、紧凑控件 |
| `radius-medium` | `6px` | 默认控件、卡片 |
| `radius-large` | `8px` | 对话框、抽屉、浮层 |
| `border-width` | `1px` | 默认边框 |
| `focus-ring-width` | `2px` | 键盘焦点环 |
| `focus-ring-offset` | `2px` | 焦点环与控件边界距离 |

同一容器内不要混用多个圆角级别。焦点环绘制在控件外侧并与背景保持足够对比度，不以改变布局尺寸的方式实现。

### 6.4 阴影与层级

| Token | 建议值 | 用途 |
|---|---|---|
| `shadow-1` | `0 1px 2px #0000000F` | 卡片、轻微抬升 |
| `shadow-2` | `0 3px 8px #0000001F` | 下拉菜单、Popover |
| `shadow-3` | `0 8px 24px #00000029` | Modal、Drawer |

平台原生窗口阴影由平台增强层处理。深色主题优先通过表面明度和边框表达层级，不能只增加黑色阴影。

视觉阴影和交互层级分别管理。层级 Token 只表达相对顺序，平台增强层负责映射原生窗口层级：

| Token | 基准值 | 用途 |
|---|---:|---|
| `layer-base` | `0` | 页面和普通内容 |
| `layer-raised` | `10` | 悬停卡片、固定工具栏 |
| `layer-sticky` | `50` | 固定标题和状态栏 |
| `layer-popup` | `100` | PopupMenu、Dropdown、Popover |
| `layer-tooltip` | `120` | Tooltip 和非交互说明 |
| `layer-drawer` | `200` | Drawer、侧边任务层 |
| `layer-modal` | `300` | Modal、ConfirmDialog |
| `layer-modal-popup` | `320` | ModalDialog 内部的 Select、PopupMenu、Tooltip |
| `layer-critical` | `400` | 必须覆盖应用内容的关键提示 |

- 子浮层必须高于其拥有者、低于后来打开的 Modal；禁止组件使用任意大数抢占层级。
- 浮层不得被普通滚动容器裁剪，除非组件规格明确要求在容器内滚动。
- Popup 根据可用空间自动翻转或平移，优先保持触发器与浮层的空间关系。
- 完整的打开、关闭、所有权与焦点恢复规则见 [`interaction.md`](interaction.md)。

## 7. 状态与交互

所有可交互组件至少定义以下状态：

```text
default → hover → pressed
        ↘ focused
        ↘ selected / checked
        ↘ disabled
        ↘ loading
        ↘ error / warning / success（适用时）
```

- Hover 只作为指针反馈，不能承载唯一信息。
- Pressed 必须立即反馈；触发动作发生在按下还是释放时由组件规格统一定义。
- Focused 必须对键盘用户清晰可见，鼠标点击后是否显示焦点环由统一焦点策略决定。
- Loading 保持控件宽度稳定并阻止重复提交。
- Disabled 不触发回调；只读状态与禁用状态必须分开。
- Selected、checked 和 pressed 是不同语义，不共用一个布尔属性替代。

状态按可用性、交互、选择和校验四个正交维度组合；组合优先级、事件触发、键盘与焦点规则见 [`interaction.md`](interaction.md)。组件规格只能补充特有行为，不能修改全局协议。

## 8. 动效

动效只用于反馈、状态过渡和层级变化。

| Token | 时长 | 用途 |
|---|---:|---|
| `motion-fast` | `100ms` | Hover、按下、颜色变化 |
| `motion-medium` | `200ms` | 展开、浮层进入、位置变化 |
| `motion-slow` | `300ms` | Modal、Drawer 和较大区域过渡 |

- 进入可略慢，退出应更快且不设置级联延迟。
- 默认缓动使用 `cubic-bezier(0.645, 0.045, 0.355, 1)`；强调回弹只在确有物理语义时使用。
- 动效不能阻塞输入或造成布局抖动。
- 必须提供全局关闭或减少动效的能力，并跟随操作系统偏好。

## 9. 图标、文案与数据

- 图标使用统一线宽、画布和视觉重量，同类操作不得混用填充与线性风格。
- 公开图标目录固定为 447 个 outlined 和 249 个 filled SVG；默认使用 outlined，filled 只用于选中、强状态或品牌图形。
- 图标从独立模块按需导入；主组件入口不得重新导出完整目录，避免未使用资源进入编译图。
- Checkbox、清除按钮等组件状态必须使用 SVG，不使用 `✓`、`×`、`−` 等字体字符模拟图标。
- 无文字图标按钮必须提供 Tooltip 和可访问名称。
- 操作文案使用动词，状态文案说明结果；避免“确定”“好的”等脱离上下文的文字。
- 中文、英文、长文本和空值都必须在 Gallery 验证。
- 时间、金额、百分比和文件大小由统一格式化层生成，组件只负责显示。

图标保留上游 SVG `viewBox`，由 `Icon` 统一映射到 12、16、20、24px 逻辑显示尺寸。文件名使用 kebab-case，Slint 资源全局使用 `PascalCase + Outlined/ Filled`，例如 `check.svg` / `CheckOutlined`。方向性图标必须声明 RTL 下是否镜像；复杂图形需要分别检查 12px 和 16px 的可辨识度，不能只验收 24px。

完整引入、许可、导入方式、体积边界和更新流程见 [`../docs/iconography.md`](../docs/iconography.md)。

文案、空值、数字、日期、时区、本地化、RTL 和脱敏规则见 [`content-and-localization.md`](content-and-localization.md)。

## 10. 主题实现约束

- `Theme` 只暴露语义 Token；基础色阶可放在内部全局对象中。
- 每个顶层窗口都必须由宿主同步主题，不能假设 Slint `global` 跨窗口自动共享实例。
- 主题切换不得重建业务状态；切换过程中不出现未主题化的中间帧。
- 组件只消费 Token，不根据主题名称编写条件分支。
- 新增组件前优先复用已有语义 Token；新增 Token 必须说明无法复用的原因。

宿主必须为每个顶层窗口提供同一套全局环境：

```text
theme: light | dark | high-contrast
density: compact | regular | comfortable
locale: BCP 47 language tag
direction: ltr | rtl
reduced-motion: bool
text-scale: number
window-size-class: narrow | standard | wide
```

主题与密度允许窗口级覆盖；Locale、Direction 和无障碍设置默认继承应用配置。环境变更必须原子应用，不能让部分组件仍使用旧值。局部主题只用于嵌入式区域或预览，不用于修复单个组件样式。

## 11. 官方参考

- [Ant Design 设计价值观](https://ant.design/docs/spec/values/)
- [Ant Design 色彩规范](https://ant.design/docs/spec/colors/)
- [Ant Design 字体规范](https://ant.design/docs/spec/font/)
- [Ant Design 布局规范](https://ant.design/docs/spec/layout/)
- [Ant Design 间距与亲密性](https://ant.design/docs/spec/proximity/)
- [Ant Design 动效规范](https://ant.design/docs/spec/motion/)
- [Ant Design 图标规范](https://ant.design/docs/spec/icon/)
- [Ant Design 深色模式](https://ant.design/docs/spec/dark/)
- [Ant Design 阴影规范](https://ant.design/docs/spec/shadow/)
- [Ant Design 主题与 Design Token](https://ant.design/docs/react/customize-theme/)

这些页面是设计依据，不是运行时依赖。Ant Design 文档升级时，SlintUI 不自动跟随；任何基线调整都必须经过 Gallery、截图和兼容性评审。
