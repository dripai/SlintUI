# Tooltip

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/tooltip.slint`。公开名称：`Tooltip`。

用于为非自解释控件提供简短补充说明。重要错误、操作结果或必须阅读的信息必须直接显示，不能只放 Tooltip。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `content` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `Tooltip` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：不直接消费 Theme Token。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

悬停定位、显示、隐藏和关闭由 Slint 原生 Tooltip 处理；`enabled = false` 时不提供说明。组件没有 Selected、Loading 或错误状态，也不拦截目标控件输入。

### 无障碍与本地化

目标控件仍必须具有独立可访问名称，Tooltip 不能替代名称。内容由调用方本地化并保持简短；复杂 Markdown、交互内容和焦点移动不属于 Tooltip。

### 验证、宿主职责与限制

Gallery“数据展示 / Tooltip”页提供独立悬停目标；原生悬停在软件渲染冒烟中编译验证。Slint 1.17.1 未公开延迟/偏移，也不能让任意 FocusScope 自动触发，因此焦点触发保持明确限制。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
