# Slider

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/slider.slint`。公开名称：`Slider`。

用于连续或步进范围的单值调整；精确整数录入使用 NumberInput，多值范围等待真实产品协议。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `value` | `in-out` | `float` | `0` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSlider |
| `minimum` | `in` | `float` | `0` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSlider |
| `maximum` | `in` | `float` | `100` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSlider |
| `step` | `in` | `float` | `1` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSlider |
| `orientation` | `in` | `orientation` | `horizontal` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSlider |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件；继承自 Slint NativeSlider |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `value: float` | 组件接受一个不同的新值后触发一次；继承自 Slint NativeSlider |
| `released` | `value: float` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范；继承自 Slint NativeSlider |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `set-value()` | `next: float` | 当前实现约束：`if (root.enabled && root.value != clamp(next, root.minimum, root.maximum)) { root.value = clamp(next, root.minimum, root.maximum); root.changed(root.value); }` |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：`orientation`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.icon-size-large`、`Theme.slider-min-width`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Hover、Pressed、Focused、Disabled、最小/最大值、步长和水平/垂直；程序化值夹取到范围且同值不重复回调。

### 无障碍与本地化

复用原生 slider 角色、范围、步长与增减动作；值格式和单位由外部 Label 或宿主格式化。

### 验证、宿主职责与限制

Gallery 展示正常和禁用；Harness 覆盖重复、越界和禁用。视觉使用 Slint 原生控件，颜色受当前原生样式能力约束。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
