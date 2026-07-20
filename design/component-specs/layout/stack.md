# Stack

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/stack.slint`。公开名称：`Stack`。

用于按统一间距垂直排列少量内容。水平排列使用 `Space`；需要虚拟化或可滚动数据时使用对应容器，不把 Stack 当列表控件。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `gap` | `in` | `length` | `Theme.space-2` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

没有组件状态、指针、键盘或焦点行为；子项保留各自交互和阅读顺序。Stack 不创建额外可访问分组。

### 无障碍与本地化

没有组件状态、指针、键盘或焦点行为；子项保留各自交互和阅读顺序。Stack 不创建额外可访问分组。

布局不内置文案，RTL 文本由子项处理。Gallery 各页面均使用 Stack，并覆盖密度和缩放。Slint 1.17.1 稳定布局不能运行时切换 `@children` 方向，未采用实验性 Flexbox。

### 验证、宿主职责与限制

布局不内置文案，RTL 文本由子项处理。Gallery 各页面均使用 Stack，并覆盖密度和缩放。Slint 1.17.1 稳定布局不能运行时切换 `@children` 方向，未采用实验性 Flexbox。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
