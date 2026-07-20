# Link

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/link.slint`。公开名称：`Link`。

用于请求宿主导航到目标；普通命令使用 Button。组件不直接打开浏览器或访问平台 API。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `target` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `external` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `activated` | `target: string` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `无` | 当前实现约束：`if (enabled && target != "") { root.activated(target); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Text`、`Image`、`TouchArea`、`FocusScope`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.color-primary`、`Theme.color-primary-hover`、`Theme.control-height-small`、`Theme.fill-hover`、`Theme.fill-pressed`、`Theme.font-size-body`、`Theme.icon-size-x-small`、`Theme.radius-small`、`Theme.space-1`、`Theme.text-disabled`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Hover、Pressed、Focused、Disabled 和外部链接标识；空目标和禁用状态不回调，Enter/Space 与点击一致。

### 无障碍与本地化

Slint 1.17.1 无公开 link 角色，当前使用 button 角色并把 target 作为说明；宿主应提供清晰的本地化链接文字。

### 验证、宿主职责与限制

Gallery 展示内部、外部和禁用链接；Harness 覆盖重复激活与禁用。导航、权限和安全策略完全由宿主处理。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
