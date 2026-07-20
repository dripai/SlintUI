# Surface

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/surface.slint`。公开名称：`Surface`。

用于表达 layout、surface、elevated、control 四类语义表面。结构化标题内容使用 `Card`；平台原生窗口阴影不由 Surface 模拟。

## 公开 API

### 数据类型与枚举

#### `SurfaceVariant`

| 字段或值 | 类型/语义 |
|---|---|
| `layout` | `枚举值` |
| `surface` | `枚举值` |
| `elevated` | `枚举值` |
| `control` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `variant` | `in` | `SurfaceVariant` | `SurfaceVariant.surface` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `bordered` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `radius` | `in` | `length` | `Theme.radius-medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `content-padding` | `in` | `length` | `0px` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `Rectangle` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`variant`、`bordered`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-elevated`、`Theme.bg-layout`、`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.radius-medium`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

背景、边框和圆角随 Theme 更新，子内容区域扣除统一内边距。

### 无障碍与本地化

Surface 本身不提供语义或文案；需要分组语义时由 Card、Toolbar 或调用方容器声明。Gallery“布局 / Surface”页配合全局主题与密度覆盖各表面层级；非交互组件通过编译与截图验收。

### 验证、宿主职责与限制

Surface 本身不提供语义或文案；需要分组语义时由 Card、Toolbar 或调用方容器声明。Gallery“布局 / Surface”页配合全局主题与密度覆盖各表面层级；非交互组件通过编译与截图验收。

Slint 核心没有通用模糊阴影，层级通过背景和边框表达。遵循 [`foundations.md`](../../foundations.md) 与 [`accessibility.md`](../../accessibility.md)。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
