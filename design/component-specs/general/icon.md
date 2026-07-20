# Icon

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/icon.slint`。公开名称：`Icon`。

用于单色操作、状态和对象图标。图标按钮使用 `IconButton`；纯装饰图像直接使用 `Image { accessible-role: none; }`。不支持需要两种独立 Theme 颜色的 Two Tone 图标。

## 公开 API

### 数据类型与枚举

#### `IconSize`

| 字段或值 | 类型/语义 |
|---|---|
| `x-small` | `枚举值` |
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `size` | `in` | `IconSize` | `IconSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `color` | `in` | `color` | `Theme.icon-default` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.icon-default`、`Theme.icon-size-large`、`Theme.icon-size-medium`、`Theme.icon-size-small`、`Theme.icon-size-x-small`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Icon 不接收输入或焦点，没有控件状态。固定为 `image` 角色，信息图标必须提供已本地化名称；颜色只来自 Theme 语义 Token。

### 验证、宿主职责与限制

Gallery“通用 / Icon”页展示尺寸语义，“图标 / Outlined、Filled”页浏览完整目录；自动测试核对 696 个资源与模块一一对应。方向图标不自动镜像，由具体导航组件按语义选择。详见 [`../docs/iconography.md`](../../../docs/iconography.md) 及四份全局规范。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
