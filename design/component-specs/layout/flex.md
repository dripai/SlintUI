# Flex

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/flex.slint`。公开名称：`Flex`。

用于水平的增长、收缩、主轴与交叉轴对齐；垂直排列使用 Stack，需要显式二维单元格时使用 Grid。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `gap` | `in` | `length` | `Theme.space-2` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

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

布局原语不接收输入；验证默认间距、不同 stretch、长文本、密度、RTL 与缩放。

### 无障碍与本地化

不增加语义层，子项自行声明名称和焦点。RTL 只影响子项内容，不自动反转任意 `@children` 顺序。

### 验证、宿主职责与限制

Gallery“布局 / Flex”展示 1:2 增长比例；编译与截图覆盖布局。Slint 稳定 API 不支持运行时换行或方向切换。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
