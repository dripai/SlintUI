# ScrollArea

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/scroll-area.slint`。公开名称：`ScrollArea`。

用于普通内容滚动。超长数据集合必须使用后续虚拟化 `List`/`Table`，不能用 ScrollArea 实例化全部数据后作为替代。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `viewport-width` | `in-out` | `length` | `绑定：view.viewport-width` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `viewport-height` | `in-out` | `length` | `绑定：view.viewport-height` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `绑定：view.enabled` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `mouse-drag-pan-enabled` | `in` | `bool` | `绑定：view.mouse-drag-pan-enabled` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `has-focus` | `out` | `bool` | `绑定：view.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `StandardScrollView` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：不直接消费 Theme Token。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

滚轮、滚动条、键盘滚动、焦点和禁用行为直接继承标准 `ScrollView`。组件不添加第二套滚动状态或回退路径。

### 无障碍与本地化

内容语义和文案由子项提供，滚动不改变阅读顺序；方向与平台滚动行为由 Slint 处理。Gallery“布局”页和主内容区覆盖滚动环境，编译与软件渲染冒烟验证创建路径。

### 验证、宿主职责与限制

内容语义和文案由子项提供，滚动不改变阅读顺序；方向与平台滚动行为由 Slint 处理。Gallery“布局”页和主内容区覆盖滚动环境，编译与软件渲染冒烟验证创建路径。

会实例化全部子元素，不提供虚拟化。遵循 [`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md) 和 [`content-and-localization.md`](../../content-and-localization.md)。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
