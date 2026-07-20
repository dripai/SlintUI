# Grid

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/grid.slint`。公开名称：`Grid`。

用于显式行列约束的规则网格；连续单轴布局使用 Flex 或 Stack，不作为网页 24 栅格替代品。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `horizontal-gap` | `in` | `length` | `Theme.space-3` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `vertical-gap` | `in` | `length` | `Theme.space-3` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.space-3`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

布局原语不接收输入；验证多行、多列、最小尺寸、伸展和缩放。

### 无障碍与本地化

不改变子项语义或朗读顺序；源码顺序必须与视觉和键盘顺序一致。

### 验证、宿主职责与限制

Gallery 展示 2×2 规则网格；编译与截图验收。列数和单元格位置受 Slint 编译期布局约束，组件不做运行时自动排布。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
