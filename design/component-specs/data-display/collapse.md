# Collapse

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/collapse.slint`。公开名称：`Collapse`。

用于按标题展开短内容区；主要导航不使用 Collapse，复杂延迟内容由宿主控制模型。

## 公开 API

### 数据类型与枚举

#### `CollapseItem`

| 字段或值 | 类型/语义 |
|---|---|
| `title` | `string` |
| `content` | `string` |
| `enabled` | `bool` |
| `expanded` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in-out` | `[CollapseItem]` | `类型默认值` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `index:int,expanded:bool` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `toggle()` | `index:int` | 当前实现约束：`if(enabled&&index>=0&&index<items.length&&items[index].enabled){root.items[index].expanded=!root.items[index].expanded;root.changed(index,root.items[index].expanded);}` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`HorizontalLayout`、`Image`、`Text`、`Rectangle` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-secondary`、`Theme.border-width`、`Theme.direction`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.icon-size-x-small`、`Theme.radius-medium`、`Theme.row-height`、`Theme.space-2`、`Theme.space-3`、`Theme.space-8`、`Theme.text-disabled`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Collapsed、Expanded、Hover、Focused semantics、Disabled、长标题/内容和越界；禁用与越界不回调。手风琴式互斥展开由宿主更新模型，组件不暴露无效开关。

### 无障碍与本地化

标题暴露 button、expandable/expanded；内容紧随标题，RTL 使用方向图标。标题与内容由宿主本地化。

### 验证、宿主职责与限制

Gallery 展示展开、折叠和禁用项；Harness 覆盖重复展开、禁用与越界。Slint 1.17.1 无通用动态 slot 模型，首期内容为字符串模型。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
