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
| `blue-4` | `#69B1FF` | 深色主题悬停 |
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

### 3.3 浅色主题

| 语义 Token | 色值 |
|---|---:|
| `bg-layout` | `#F5F5F5` |
| `bg-surface` | `#FFFFFF` |
| `bg-elevated` | `#FFFFFF` |
| `bg-control` | `#FFFFFF` |
| `text-primary` | `#000000E0` |
| `text-secondary` | `#000000A6` |
| `text-disabled` | `#00000040` |
| `border-default` | `#D9D9D9` |
| `border-secondary` | `#0505050F` |
| `fill-hover` | `#0000000F` |
| `fill-pressed` | `#0000001A` |
| `overlay-mask` | `#00000073` |

### 3.4 深色主题

| 语义 Token | 色值 |
|---|---:|
| `bg-layout` | `#000000` |
| `bg-surface` | `#141414` |
| `bg-elevated` | `#1F1F1F` |
| `bg-control` | `#141414` |
| `text-primary` | `#FFFFFFD9` |
| `text-secondary` | `#FFFFFFA6` |
| `text-disabled` | `#FFFFFF40` |
| `border-default` | `#424242` |
| `border-secondary` | `#FDFDFD1F` |
| `fill-hover` | `#FFFFFF14` |
| `fill-pressed` | `#FFFFFF1F` |
| `overlay-mask` | `#000000A6` |

深色主题不是浅色主题的简单反相。阴影、边框、填充和功能色需要分别验收；浮层必须比承载它的表面更高一级。

### 3.5 高对比度主题

高对比度主题使用独立语义 Token，首期不承诺固定色值，实施时遵循以下约束：

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

## 6. 尺寸、圆角、边框与阴影

### 6.1 控件尺寸

| Size | 高度 | 用途 |
|---|---:|---|
| `small` | `24px` | 高密度工具栏、表格内操作 |
| `medium` | `32px` | 默认按钮、输入和选择控件 |
| `large` | `40px` | 低密度表单和关键操作 |

24px 控件只用于鼠标密集场景；触摸或可访问性模式应使用 40px 或更大的命中区域。图标标准尺寸为 12、16、20、24px，默认控件图标为 16px。

### 6.2 圆角与边框

| Token | 数值 | 用途 |
|---|---:|---|
| `radius-small` | `4px` | 标签、紧凑控件 |
| `radius-medium` | `6px` | 默认控件、卡片 |
| `radius-large` | `8px` | 对话框、抽屉、浮层 |
| `border-width` | `1px` | 默认边框 |
| `focus-ring-width` | `2px` | 键盘焦点环 |

同一容器内不要混用多个圆角级别。焦点环绘制在控件外侧并与背景保持足够对比度，不以改变布局尺寸的方式实现。

### 6.3 阴影层级

| Token | 建议值 | 用途 |
|---|---|---|
| `shadow-1` | `0 1px 2px #0000000F` | 卡片、轻微抬升 |
| `shadow-2` | `0 3px 8px #0000001F` | 下拉菜单、Popover |
| `shadow-3` | `0 8px 24px #00000029` | Modal、Drawer |

平台原生窗口阴影由平台增强层处理。深色主题优先通过表面明度和边框表达层级，不能只增加黑色阴影。

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
- 无文字图标按钮必须提供 Tooltip 和可访问名称。
- 操作文案使用动词，状态文案说明结果；避免“确定”“好的”等脱离上下文的文字。
- 中文、英文、长文本和空值都必须在 Gallery 验证。
- 时间、金额、百分比和文件大小由统一格式化层生成，组件只负责显示。

## 10. 主题实现约束

- `SlintTheme` 只暴露语义 Token；基础色阶可放在内部全局对象中。
- 每个顶层窗口都必须由宿主同步主题，不能假设 Slint `global` 跨窗口自动共享实例。
- 主题切换不得重建业务状态；切换过程中不出现未主题化的中间帧。
- 组件只消费 Token，不根据主题名称编写条件分支。
- 新增组件前优先复用已有语义 Token；新增 Token 必须说明无法复用的原因。

## 11. 官方参考

- [Ant Design 设计价值观](https://ant.design/docs/spec/values/)
- [Ant Design 色彩规范](https://ant.design/docs/spec/colors/)
- [Ant Design 字体规范](https://ant.design/docs/spec/font/)
- [Ant Design 布局规范](https://ant.design/docs/spec/layout/)
- [Ant Design 间距与亲密性](https://ant.design/docs/spec/proximity/)
- [Ant Design 动效规范](https://ant.design/docs/spec/motion/)
- [Ant Design 主题与 Design Token](https://ant.design/docs/react/customize-theme/)

这些页面是设计依据，不是运行时依赖。Ant Design 文档升级时，SlintUI 不自动跟随；任何基线调整都必须经过 Gallery、截图和兼容性评审。
