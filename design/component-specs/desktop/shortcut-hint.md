# ShortcutHint

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/shortcut-hint.slint`。公开名称：`ShortcutHint`。

用于菜单、命令或帮助文字旁显示跨平台快捷键；不负责注册、解析或执行快捷键。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `shortcut` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `shortcut` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |

## 视觉规范

### 组成结构

当前实现由 `Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.border-default`、`Theme.border-width`、`Theme.font-size-caption`、`Theme.font-weight-medium`、`Theme.radius-small`、`Theme.space-2`、`Theme.tag-height`、`Theme.text-disabled`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖默认、Disabled、长组合、不同平台符号和缩放。

### 无障碍与本地化

使用 text 角色；宿主根据平台提供 `Ctrl`、`⌘` 等最终显示和朗读名称。

### 验证、宿主职责与限制

Gallery 展示 Windows/macOS 风格和禁用；编译与截图验收。不内建平台探测，避免平台 API 泄漏到核心层。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
