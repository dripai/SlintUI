# Pagination

成熟度：`Alpha`。源码：`crates/slint-ui/ui/navigation/pagination.slint`。公开名称：`Pagination`。

控制服务端或宿主分页数据，不负责数据加载。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `current-page` | `in-out` | `int` | `1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `page-count` | `in` | `int` | `1` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `page-size-index` | `in-out` | `int` | `0` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `page-size-options` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `previous-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `next-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `page-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `page: int` | 组件接受一个不同的新值后触发一次 |
| `page-size-changed` | `index: int, value: string` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select-page()` | `page: int` | 当前实现约束：`let next = clamp(page, 1, max(1, page-count)); if (next != current-page) { root.current-page = next; root.changed(next); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Button`、`Text`、`Select` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`page-size-index`、`page-size-options`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.control-height-medium`、`Theme.direction`、`Theme.font-size-body`、`Theme.space-2`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

上一页、下一页按边界禁用；程序化页码被限制到有效范围；RTL 图标镜像。

### 无障碍与本地化

根为 groupbox，方向按钮和页码选择均可命名；页大小文本由调用方格式化。

### 验证、宿主职责与限制

Gallery 和 Harness 覆盖页码、边界和页大小。当前仅显示当前页/总页数，不生成数字页码窗口或跳页输入。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
