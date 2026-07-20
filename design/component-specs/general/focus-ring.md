# FocusRing

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/focus-ring.slint`。公开名称：`FocusRing`。

为自定义控件绘制统一键盘焦点环。它只负责视觉，不获取焦点；实际焦点必须由 `FocusScope` 或原生控件管理。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `active` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `radius` | `in` | `length` | `Theme.radius-medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `target-width` | `in` | `length` | `0px` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `target-height` | `in` | `length` | `0px` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：`active`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.focus-outline`、`Theme.focus-ring-offset`、`Theme.focus-ring-width`、`Theme.radius-medium`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

仅有 active/inactive 视觉状态，不接收指针或键盘。焦点环位于目标外侧，不改变布局尺寸；颜色、宽度和偏移来自 Theme。

### 无障碍与本地化

FocusRing 不进入可访问树；可访问焦点仍由目标控件声明。Gallery“布局 / FocusRing”页展示 active 状态，高对比度截图验证可见性；遵循 [`interaction.md`](../../interaction.md) 与 [`accessibility.md`](../../accessibility.md)。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
