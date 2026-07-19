# SlintUI 组件清单与实施优先级

本文定义 SlintUI 的目标组件范围。分类参考 Ant Design 官方组件总览，并针对 Slint 桌面应用补充窗口、工具栏、快捷键、右键菜单和平台能力。清单是规划基线，不表示组件已经实现。

## 1. 优先级定义

| 优先级 | 含义 | 进入条件 |
|---|---|---|
| P0 | 首期基础 | Gallery 和首个真实设置页、工具栏必需 |
| P1 | 常用桌面能力 | 至少两个明确页面或产品场景需要 |
| P2 | 扩展能力 | 需求明确后实现，不为追求数量提前建设 |
| 平台 | 平台增强 | 依赖 Windows、macOS 或 Linux API，不进入核心组件层 |

每个组件的状态为“待实现”“实现中”“已完成”之一。当前仓库处于设计基线阶段，以下条目均为待实现。

## 2. 基础与主题

| 组件 | 优先级 | 目标位置 | 主要职责 |
|---|---:|---|---|
| `SlintTheme` | P0 | `ui/tokens/` | 浅色、深色、高对比度语义 Token |
| `SlintTypography` | P0 | `ui/primitives/typography.slint` | 标题、正文、标签、辅助文本 |
| `SlintIcon` | P0 | `ui/primitives/icon.slint` | 统一尺寸、颜色和可访问名称 |
| `SlintSurface` | P0 | `ui/primitives/surface.slint` | 背景、边框、圆角和层级表面 |
| `SlintDivider` | P0 | `ui/primitives/divider.slint` | 水平、垂直分隔 |
| `SlintFocusRing` | P0 | `ui/primitives/focus-ring.slint` | 统一键盘焦点反馈 |
| `SlintOverlay` | P1 | `ui/primitives/overlay.slint` | 浮层遮罩和输入拦截 |
| `SlintScrollArea` | P1 | `ui/primitives/scroll-area.slint` | 统一滚动条、滚动边界和键盘滚动 |

## 3. 操作组件

| 组件 | 优先级 | 变体或关键能力 |
|---|---:|---|
| `SlintButton` | P0 | default、primary、danger、text、link；small、medium、large；loading |
| `SlintIconButton` | P0 | 图标操作、Tooltip、可访问名称 |
| `SlintToolButton` | P0 | 工具栏密度、checked、分组 |
| `SlintButtonGroup` | P1 | 连续按钮的圆角、边框和键盘导航 |
| `SlintToggleButton` | P1 | 独立开关式操作，不替代 Checkbox |
| `SlintSplitButton` | P2 | 主操作加下拉菜单 |
| `SlintLink` | P1 | 文本链接、外部链接、禁用状态 |

## 4. 布局与容器

| 组件 | 优先级 | 主要职责 |
|---|---:|---|
| `SlintStack` | P0 | 水平或垂直排列、统一 gap |
| `SlintSpace` | P0 | 内容间距、换行和对齐 |
| `SlintCard` | P0 | 标题、内容、操作区和边框层级 |
| `SlintFormRow` | P0 | 标签、控件、帮助和错误信息对齐 |
| `SlintSettingsSection` | P0 | 设置页标题、说明和表单行组合 |
| `SlintToolbar` | P0 | 工具分组、分隔、溢出和键盘访问 |
| `SlintFlex` | P1 | 可增长、收缩、对齐的通用布局 |
| `SlintGrid` | P1 | 规则网格和响应窗口宽度 |
| `SlintSplitPane` | P1 | 可拖动分栏、最小尺寸和折叠 |
| `SlintPanel` | P1 | 可复用的标题栏与内容区容器 |
| `SlintAppShell` | P1 | 标题区、导航区、内容区和状态区骨架 |

## 5. 导航

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `SlintTabs` | P1 | 标签切换、关闭、溢出、键盘导航 |
| `SlintSegmentedControl` | P0 | 少量互斥视图或模式切换 |
| `SlintMenu` / `SlintMenuItem` | P1 | 原生菜单行为、分组、快捷键、子菜单 |
| `SlintContextMenu` | P1 | 指针位置打开、焦点恢复 |
| `SlintDropdown` | P1 | 触发器、定位、关闭策略 |
| `SlintBreadcrumb` | P2 | 层级路径和溢出 |
| `SlintSteps` | P2 | 有顺序的多步任务 |
| `SlintPagination` | P2 | 大数据分页和页大小选择 |
| `SlintNavigationRail` | P2 | 桌面侧边主导航 |
| `SlintCommandPalette` | P2 | 命令搜索、快捷键和最近命令 |

菜单、Dropdown 和 ContextMenu 优先包装 Slint 原生能力，不自建窗口、焦点捕获和关闭策略。

## 6. 数据录入

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `SlintTextField` | P0 | 前后缀、清除、校验、只读、密码模式 |
| `SlintTextArea` | P1 | 多行、字符计数、滚动和调整尺寸 |
| `SlintSearchField` | P1 | 搜索、清除、提交和延迟输入 |
| `SlintNumberInput` | P1 | 步进、范围、精度和格式化 |
| `SlintCheckbox` | P0 | checked、unchecked、indeterminate |
| `SlintCheckboxGroup` | P1 | 组选项和统一错误提示 |
| `SlintRadio` / `SlintRadioGroup` | P1 | 互斥选择和方向键导航 |
| `SlintSwitch` | P0 | 立即生效的开关状态 |
| `SlintSelect` | P0 | 单选、搜索、键盘选择和空状态 |
| `SlintComboBox` | P1 | 可编辑输入加候选列表 |
| `SlintMultiSelect` | P2 | 多选、Tag、搜索和溢出 |
| `SlintAutoComplete` | P2 | 输入建议，不等同于 Select |
| `SlintSlider` | P1 | 单值、范围、步长、键盘调整 |
| `SlintDatePicker` | P2 | 本地化日期、范围和键盘输入 |
| `SlintTimePicker` | P2 | 本地化时间、步长和键盘输入 |
| `SlintColorPicker` | P2 | 色彩输入、透明度和格式 |
| `SlintFilePicker` | P1 | 统一入口；文件对话框由平台或宿主提供 |
| `SlintForm` | P1 | 校验、提交、错误聚合和焦点定位 |

输入组件必须区分空值、无效值和未提交值。格式化层与组件显示层分离，不静默修正用户输入。

## 7. 数据展示

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `SlintLabel` | P0 | 截断、换行、助记键和 Tooltip |
| `SlintBadge` | P1 | 状态点、计数和上限 |
| `SlintTag` | P1 | 状态、可关闭、可选择 |
| `SlintAvatar` | P2 | 图片、文字、回退图标 |
| `SlintList` / `SlintListItem` | P1 | 选择、虚拟化预留、键盘导航 |
| `SlintTable` | P1 | 列、排序、选择、固定表头、空状态 |
| `SlintDataGrid` | P2 | 大数据、编辑、列宽和虚拟化 |
| `SlintTree` | P1 | 展开、选择、懒加载、键盘导航 |
| `SlintDescriptionList` | P1 | 名称和值的详情展示 |
| `SlintStatistic` | P2 | 数值、趋势和等宽数字 |
| `SlintCollapse` | P1 | 展开区域和焦点顺序 |
| `SlintTimeline` | P2 | 时间节点和状态 |
| `SlintImage` | P2 | 占位、失败和适配方式 |
| `SlintCalendar` | P2 | 月视图、选择和本地化 |
| `SlintEmptyState` | P0 | 无数据、无结果、无权限三类语义 |
| `SlintTooltip` | P0 | 延迟、定位、快捷键和可访问说明 |
| `SlintPopover` | P1 | 交互内容、焦点和关闭策略 |

`SlintTable` 首期不承诺虚拟化和单元格编辑；这些能力在性能需求明确后进入 `SlintDataGrid`，避免一个组件承担两套复杂行为。

## 8. 反馈与浮层

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `SlintAlert` | P1 | info、success、warning、error；可关闭 |
| `SlintToast` | P1 | 短暂反馈、队列、自动关闭 |
| `SlintNotification` | P2 | 带标题和操作的持久通知 |
| `SlintDialog` | P1 | 焦点陷阱、默认按钮、Esc 和关闭确认 |
| `SlintConfirmDialog` | P1 | 普通、危险确认和不可逆提示 |
| `SlintDrawer` | P2 | 侧边任务、遮罩和焦点恢复 |
| `SlintProgressBar` | P0 | 确定进度和不确定进度 |
| `SlintProgressRing` | P1 | 紧凑区域进度 |
| `SlintSpinner` | P0 | 局部短时加载 |
| `SlintSkeleton` | P2 | 首次内容加载占位 |
| `SlintResultState` | P1 | 成功、失败、警告和后续操作 |
| `SlintLoadingOverlay` | P1 | 区域阻塞、说明和取消能力 |

Toast 不用于必须阅读、必须操作或包含重要错误详情的消息。对话框和浮层优先使用 Slint 原生窗口与弹层能力。

## 9. 桌面窗口与平台增强

| 组件或能力 | 优先级 | 层级 | 关键能力 |
|---|---:|---|---|
| `SlintStatusBar` | P0 | 核心 | 状态、进度、连接信息和临时提示 |
| `SlintShortcutHint` | P1 | 核心 | 跨平台快捷键显示 |
| `SlintTitleBar` | P1 | 核心外观 | 标题、拖动区、窗口操作槽位 |
| 窗口拖动与缩放 | 平台 | 平台增强 | 原生命中测试和系统行为 |
| 原生窗口阴影 | 平台 | 平台增强 | 跟随系统合成器 |
| 系统托盘 | 平台 | 平台增强 | 菜单、通知和生命周期 |
| 全局快捷键 | 平台 | 平台增强 | 注册、冲突和注销 |
| 原生文件对话框 | 平台 | 平台增强 | 打开、保存、目录和过滤器 |
| 系统通知 | 平台 | 平台增强 | 权限、动作和激活 |

平台增强包必须实现统一接口。核心组件不能通过条件判断直接调用操作系统 API。

## 10. 不纳入首期或不直接照搬的组件

以下 Ant Design 类别不直接进入首期核心范围：

- FloatButton、Affix、Anchor：偏网页滚动与悬浮交互，桌面场景不足。
- Carousel、Tour、Watermark、QRCode、Rate、Mentions、Transfer：等待真实产品需求。
- Masonry、24 栅格页面模板：可参考布局思想，不作为桌面核心组件的前置条件。
- App、ConfigProvider、Util：属于 React 运行时或工具层概念，不映射为可视组件。
- ProComponents：属于更高层业务组合，后续按 SlintUI 自身需求建设。

缺少明确场景时返回“未实现”，不使用相似组件静默替代。

## 11. 推荐实施顺序

### 里程碑 A：基础视觉与 Gallery 骨架

1. Theme、Typography、Icon、Surface、Divider、FocusRing。
2. Stack、Space、Card、ScrollArea。
3. Gallery 导航、主题切换、缩放预览和状态展示模板。

### 里程碑 B：首批可交互组件

1. Button、IconButton、ToolButton、Toolbar。
2. Checkbox、Switch、TextField、Select、SegmentedControl。
3. FormRow、SettingsSection、Tooltip、Progress、Spinner、EmptyState、StatusBar。

### 里程碑 C：真实产品验证

1. 使用首个真实设置页验证表单、错误、长文本和焦点顺序。
2. 使用首个真实工具栏验证密度、快捷键、Tooltip 和选中态。
3. 修复公共组件后发布固定版本，业务项目不复制组件源码。

### 里程碑 D：常用桌面能力

按真实需求加入 Menu、ContextMenu、Tabs、Table、Tree、Dialog、Toast、SplitPane、Form 和平台增强能力。

## 12. 单组件文档模板

每个 `design/component-specs/<component>.md` 必须包含：

1. 用途、不适用场景和替代组件。
2. 属性、类型、默认值、回调和枚举。
3. Variant、Size 和全部交互状态。
4. 鼠标、键盘、触摸板和焦点行为。
5. 可访问名称、角色、状态和错误提示。
6. 中文、英文、长文本、空值和本地化策略。
7. 浅色、深色、高对比度和缩放验收图。
8. Slint 原生能力复用情况及已知限制。
9. Gallery 示例、交互测试和截图基线。

## 13. 组件完成定义

组件同时满足以下条件才可标记为“已完成”：

- 规格文档和稳定公开 API 已评审。
- 默认、Hover、Pressed、Focused、Selected、Disabled、Loading 和错误状态按适用范围实现。
- 键盘操作、焦点顺序和可访问名称可用。
- Gallery 覆盖全部变体、尺寸、主题、长文本和缩放比例。
- 通过编译、交互测试、UTF-8 检查和截图人工评审。
- 没有业务概念、散落色值或平台 API 泄漏到核心层。

## 14. 官方参考

- [Ant Design 组件总览](https://ant.design/components/overview/)
- [Ant Design 主题与 Design Token](https://ant.design/docs/react/customize-theme/)
- [Ant Design 数据录入设计原则](https://ant.design/docs/spec/data-entry/)
- [Ant Design 数据展示设计原则](https://ant.design/docs/spec/data-display/)
- [Ant Design 按钮设计原则](https://ant.design/docs/spec/buttons/)

组件名称只用于建立共同语义。实现前仍须核对 Slint 标准组件与平台行为，不能假设 React 组件 API 可以直接移植。
