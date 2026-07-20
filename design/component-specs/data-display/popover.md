# Popover

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/popover.slint`。公开名称：`Popover`。

用于触发器旁的短交互说明和操作；纯说明使用 Tooltip，菜单命令使用 Dropdown，模态任务使用 Dialog。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `trigger-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `message` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `action-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `close-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `open` | `out` | `bool` | `popup.is-open` | 组件派生 | 打开状态的所有权和关闭原因见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `action-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `show()` | `无` | 当前实现约束：`if(enabled){popup.show();}` |

## 视觉规范

### 组成结构

当前实现由 `Button`、`Rectangle`、`HorizontalLayout` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-width`、`Theme.control-height-medium`、`Theme.control-min-width`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.popover-width`、`Theme.radius-large`、`Theme.space-2`、`Theme.space-3`、`Theme.space-6`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Closed、Open、Focused、Disabled、Action、Close、点击外部和 Escape；禁用不打开或激活。

### 无障碍与本地化

触发器暴露 expandable/expanded，弹层转移焦点；所有显示文字必须由调用方本地化。

### 验证、宿主职责与限制

Gallery 展示标题、长说明和双操作；Harness 覆盖重复动作。Slint 1.17.1 无通用命名 slot 和跨窗口焦点恢复，首期采用显式标题/消息/操作 API。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
