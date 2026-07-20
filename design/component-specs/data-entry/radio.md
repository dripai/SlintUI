# Radio

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/radio.slint`。公开名称：`Radio`。

用于互斥选项中的单个可选项。多个 Radio 必须由 RadioGroup 或宿主共享一个选择状态；独立布尔值应使用 Checkbox。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `value` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `checked` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `value: string` | 用户选择有效且可用项目后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `无` | 当前实现约束：`if (enabled && !checked) { root.selected(value); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`FocusRing`、`Text`、`TouchArea`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`checked`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.border-default`、`Theme.border-width`、`Theme.color-primary`、`Theme.control-height-small`、`Theme.font-size-body`、`Theme.icon-size-small`、`Theme.line-height-body`、`Theme.space-1`、`Theme.space-2`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Unchecked、Checked、Hover、Focused 和 Disabled 由 checked、enabled 与焦点状态共同决定。select() 只在可用且尚未选中时发出 selected(value)，组件自身不修改 checked。

### 无障碍与本地化

使用 radio-button 角色，暴露名称、可用和选中状态；可见文字与稳定 value 分离，二者均由宿主本地化和维护。

### 验证、宿主职责与限制

方向键组内移动由 RadioGroup 的选择协议承担；单个 Radio 只处理指针、Space 和 Enter 激活。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
