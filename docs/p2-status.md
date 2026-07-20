# P2 里程碑 E 实现状态

里程碑 E 已完成 `component-inventory.md` 中全部 20 个 P2 核心组件，不包含优先级为“平台”的操作系统能力。

## 已实现范围

- 操作与导航：SplitButton、Breadcrumb、Steps、Pagination、NavigationRail、CommandPalette。
- 数据录入：MultiSelect、AutoComplete、DatePicker、TimePicker、ColorPicker。
- 数据展示：Avatar、DataGrid、Statistic、Timeline、ImageView、Calendar。
- 反馈与浮层：Notification、Drawer、Skeleton。

全部组件从 `@slint-ui/index.slint` 导出，视觉值来自 Theme Token，状态图标使用按需 SVG，业务文案、筛选、格式化和数据加载通过属性与回调注入。

## 验证范围

- Gallery 第 7 页覆盖 20 个组件的代表性组合，并继承主题、密度、预览缩放、Locale、RTL 和 reduced-motion 环境。
- 自动交互测试覆盖 SplitButton、Breadcrumb、Steps、Pagination、NavigationRail、CommandPalette、MultiSelect、AutoComplete、DatePicker、TimePicker、ColorPicker、DataGrid 和 Calendar 的公开边界。
- 58 份已实现组件规格进入结构测试；P2 软件渲染截图及 SHA-256 进入 manifest。

## Slint 1.17.1 与当前稳定边界

- CommandPalette 的全局快捷键、搜索算法和最近命令由宿主实现；核心组件只展示受控命令模型。
- MultiSelect 使用受控摘要，不内建 Tag 布局和搜索；AutoComplete 的筛选、防抖和异步请求由宿主实现。
- DatePicker/TimePicker 使用原生 Select 组合。历法、闰年、范围、12/24 小时和时区属于数据格式层，不在组件内猜测。
- DataGrid 复用 StandardTableView，并通过 `row-offset`、`total-row-count`、`range-requested` 建立宿主窗口化协议；不内建缓存、固定列和单元格编辑器。
- ImageView 的加载状态由宿主控制，因为通用图片包装层没有稳定的跨来源错误事件。
- Drawer 提供遮罩与关闭请求；焦点陷阱、所有者焦点恢复和平台模态关系没有通用稳定接口，必须由顶层宿主协调。
- Notification 是窗口内持久通知，不调用系统通知 API；系统托盘、系统通知和全局快捷键仍属于平台增强。

## 仍待实现

清单中未进入里程碑 D 的其余 P1 组件、三平台真机屏幕阅读器/真实 DPI 验证、真实业务产品接入和全部平台增强仍保持“待实现”。
