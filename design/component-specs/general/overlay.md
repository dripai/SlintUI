# Overlay

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/overlay.slint`。公开名称：`Overlay`。

为应用内浮层提供语义遮罩和外部点击拦截，不负责窗口层级、焦点陷阱或系统模态行为。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `active` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `dismiss-on-click` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `dismissed` | `无` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `dismiss()` | `无` | 当前实现约束：`if (active && dismiss-on-click) { root.dismissed(); }` |

## 视觉规范

### 组成结构

当前实现由 `TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`active`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.overlay-mask`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持 active、disabled dismissal；点击遮罩仅在允许时触发一次 dismissed。内容区域由调用方放在其上层并负责阻止事件穿透。

### 无障碍与本地化

Role 为 region，名称由宿主提供；没有内置文案。RTL 不改变遮罩行为。

### 验证、宿主职责与限制

随 Modal/Toast 组合编译和 Gallery 冒烟验证。Slint 原生 Rectangle/TouchArea 不提供平台模态语义，平台窗口所有权仍待增强层实现。

遵循全局视觉、交互、无障碍和内容本地化规范。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
