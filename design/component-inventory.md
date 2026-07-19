# SlintUI 组件清单与实施优先级

本文定义 SlintUI 的目标组件范围。分类参考 Ant Design 官方组件总览，并针对 Slint 桌面应用补充窗口、工具栏、快捷键、右键菜单和平台能力。公开组件通过 `@slint-ui` 提供命名空间，组件名称不添加 `Slint` 前缀。清单是规划基线，不表示组件已经实现。

## 1. 优先级定义

| 优先级 | 含义 | 进入条件 |
|---|---|---|
| P0 | 首期基础 | Gallery 和首个真实设置页、工具栏必需 |
| P1 | 常用桌面能力 | 至少两个明确页面或产品场景需要 |
| P2 | 扩展能力 | 需求明确后实现，不为追求数量提前建设 |
| 平台 | 平台增强 | 依赖 Windows、macOS 或 Linux API，不进入核心组件层 |

每个组件的状态为“待实现”“实现中”“已完成”之一。当前仓库已完成全部 P0 组件，并按本次任务提前完成 `ScrollArea`；P1/P2 和平台增强条目仍为待实现。逐组件状态、API 和限制见 [`component-specs/`](component-specs/README.md)。

### 1.1 当前已完成范围

| 范围 | 状态 | 说明 |
|---|---|---|
| 里程碑 A | 已完成 | Theme、基础原语、Stack、Space、Card、ScrollArea 和 Gallery 骨架 |
| 里程碑 B | 已完成 | P0 操作、输入、表单、反馈和状态栏组件 |
| 里程碑 C 的 Gallery 场景 | 已完成 | 代表性设置页、工具栏、主题、密度、缩放、Locale、RTL 和截图场景 |
| 平台无障碍与真实 DPI 验证 | 待实现 | 语义已声明；Narrator、NVDA、VoiceOver、Orca 和三平台真机矩阵尚未执行 |
| 真实业务产品接入 | 待实现 | 当前仓库没有可供迁移的外部业务页面，不虚构验证结果 |

## 2. 基础与主题

| 组件 | 优先级 | 目标位置 | 主要职责 |
|---|---:|---|---|
| `Theme` | P0 | `ui/tokens/` | 浅色、深色、高对比度语义 Token |
| `Typography` | P0 | `ui/primitives/typography.slint` | 标题、正文、标签、辅助文本 |
| `Icon` | P0 | `ui/primitives/icon.slint` | 统一尺寸、颜色和可访问名称 |
| `Surface` | P0 | `ui/primitives/surface.slint` | 背景、边框、圆角和层级表面 |
| `Divider` | P0 | `ui/primitives/divider.slint` | 水平、垂直分隔 |
| `FocusRing` | P0 | `ui/primitives/focus-ring.slint` | 统一键盘焦点反馈 |
| `Overlay` | P1 | `ui/primitives/overlay.slint` | 浮层遮罩和输入拦截 |
| `ScrollArea` | P1 | `ui/primitives/scroll-area.slint` | 统一滚动条、滚动边界和键盘滚动 |

## 3. 操作组件

| 组件 | 优先级 | 变体或关键能力 |
|---|---:|---|
| `Button` | P0 | default、primary、danger、text、link；small、medium、large；loading |
| `IconButton` | P0 | 图标操作、Tooltip、可访问名称 |
| `ToolButton` | P0 | 工具栏密度、checked、分组 |
| `ButtonGroup` | P1 | 连续按钮的圆角、边框和键盘导航 |
| `ToggleButton` | P1 | 独立开关式操作，不替代 Checkbox |
| `SplitButton` | P2 | 主操作加下拉菜单 |
| `Link` | P1 | 文本链接、外部链接、禁用状态 |

## 4. 布局与容器

| 组件 | 优先级 | 主要职责 |
|---|---:|---|
| `Stack` | P0 | 稳定垂直排列、统一 gap |
| `Space` | P0 | 稳定水平排列、间距和交叉轴对齐；自动换行受 Slint 稳定 API 限制 |
| `Card` | P0 | 标题、内容、操作区和边框层级 |
| `FormRow` | P0 | 标签、控件、帮助和错误信息对齐 |
| `SettingsSection` | P0 | 设置页标题、说明和表单行组合 |
| `Toolbar` | P0 | 工具分组、分隔和 Tab 键盘访问；溢出与 roving focus 待稳定子项接口 |
| `Flex` | P1 | 可增长、收缩、对齐的通用布局 |
| `Grid` | P1 | 规则网格和响应窗口宽度 |
| `SplitPane` | P1 | 可拖动分栏、最小尺寸和折叠 |
| `Panel` | P1 | 可复用的标题栏与内容区容器 |
| `AppShell` | P1 | 标题区、导航区、内容区和状态区骨架 |

## 5. 导航

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `Tabs` | P1 | 标签切换、关闭、溢出、键盘导航 |
| `SegmentedControl` | P0 | 少量互斥视图或模式切换 |
| `PopupMenu` / `PopupMenuItem` | P1 | 原生菜单行为、分组、快捷键、子菜单 |
| `ContextMenu` | P1 | 指针位置打开、焦点恢复 |
| `Dropdown` | P1 | 触发器、定位、关闭策略 |
| `Breadcrumb` | P2 | 层级路径和溢出 |
| `Steps` | P2 | 有顺序的多步任务 |
| `Pagination` | P2 | 大数据分页和页大小选择 |
| `NavigationRail` | P2 | 桌面侧边主导航 |
| `CommandPalette` | P2 | 命令搜索、快捷键和最近命令 |

菜单、Dropdown 和 ContextMenu 优先包装 Slint 原生能力，不自建窗口、焦点捕获和关闭策略。

## 6. 数据录入

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `TextField` | P0 | 前后缀、清除、校验、只读、密码模式 |
| `TextArea` | P1 | 多行、字符计数、滚动和调整尺寸 |
| `SearchField` | P1 | 搜索、清除、提交和延迟输入 |
| `NumberInput` | P1 | 步进、范围、精度和格式化 |
| `Checkbox` | P0 | checked、unchecked、indeterminate |
| `CheckboxGroup` | P1 | 组选项和统一错误提示 |
| `Radio` / `RadioGroup` | P1 | 互斥选择和方向键导航 |
| `Switch` | P0 | 立即生效的开关状态 |
| `Select` | P0 | 原生单选、Popup、键盘选择和空状态；搜索受 Slint 1.17.1 原生能力限制 |
| `ComboBox` | P1 | 可编辑输入加候选列表 |
| `MultiSelect` | P2 | 多选、Tag、搜索和溢出 |
| `AutoComplete` | P2 | 输入建议，不等同于 Select |
| `Slider` | P1 | 单值、范围、步长、键盘调整 |
| `DatePicker` | P2 | 本地化日期、范围和键盘输入 |
| `TimePicker` | P2 | 本地化时间、步长和键盘输入 |
| `ColorPicker` | P2 | 色彩输入、透明度和格式 |
| `FilePicker` | P1 | 统一入口；文件对话框由平台或宿主提供 |
| `Form` | P1 | 校验、提交、错误聚合和焦点定位 |

输入组件必须区分空值、无效值和未提交值。格式化层与组件显示层分离，不静默修正用户输入。

## 7. 数据展示

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `Label` | P0 | 截断、换行、助记键和 Tooltip |
| `Badge` | P1 | 状态点、计数和上限 |
| `Tag` | P1 | 状态、可关闭、可选择 |
| `Avatar` | P2 | 图片、文字、回退图标 |
| `List` / `ListItem` | P1 | 选择、虚拟化预留、键盘导航 |
| `Table` | P1 | 列、排序、选择、固定表头、空状态 |
| `DataGrid` | P2 | 大数据、编辑、列宽和虚拟化 |
| `Tree` | P1 | 展开、选择、懒加载、键盘导航 |
| `DescriptionList` | P1 | 名称和值的详情展示 |
| `Statistic` | P2 | 数值、趋势和等宽数字 |
| `Collapse` | P1 | 展开区域和焦点顺序 |
| `Timeline` | P2 | 时间节点和状态 |
| `ImageView` | P2 | 占位、失败和适配方式 |
| `Calendar` | P2 | 月视图、选择和本地化 |
| `EmptyState` | P0 | 无数据、无结果、无权限三类语义 |
| `Tooltip` | P0 | 原生悬停定位和可访问说明；延迟、偏移与焦点触发受 Slint 1.17.1 公开能力限制 |
| `Popover` | P1 | 交互内容、焦点和关闭策略 |

`Table` 首期不承诺虚拟化和单元格编辑；这些能力在性能需求明确后进入 `DataGrid`，避免一个组件承担两套复杂行为。

## 8. 反馈与浮层

| 组件 | 优先级 | 关键能力 |
|---|---:|---|
| `Alert` | P1 | info、success、warning、error；可关闭 |
| `Toast` | P1 | 短暂反馈、队列、自动关闭 |
| `Notification` | P2 | 带标题和操作的持久通知 |
| `ModalDialog` | P1 | 焦点陷阱、默认按钮、Esc 和关闭确认 |
| `ConfirmDialog` | P1 | 普通、危险确认和不可逆提示 |
| `Drawer` | P2 | 侧边任务、遮罩和焦点恢复 |
| `Progress` | P0 | 确定进度和不确定进度 |
| `ProgressRing` | P1 | 紧凑区域进度 |
| `Spinner` | P0 | 局部短时加载 |
| `Skeleton` | P2 | 首次内容加载占位 |
| `ResultState` | P1 | 成功、失败、警告和后续操作 |
| `LoadingOverlay` | P1 | 区域阻塞、说明和取消能力 |

Toast 不用于必须阅读、必须操作或包含重要错误详情的消息。对话框和浮层优先使用 Slint 原生窗口与弹层能力。

## 9. 桌面窗口与平台增强

| 组件或能力 | 优先级 | 层级 | 关键能力 |
|---|---:|---|---|
| `StatusBar` | P0 | 核心 | 状态、进度、连接信息和临时提示 |
| `ShortcutHint` | P1 | 核心 | 跨平台快捷键显示 |
| `TitleBar` | P1 | 核心外观 | 标题、拖动区、窗口操作槽位 |
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

### 里程碑 A：基础视觉与 Gallery 骨架（已完成）

1. Theme、Typography、Icon、Surface、Divider、FocusRing。
2. Stack、Space、Card、ScrollArea。
3. Gallery 导航、主题切换、缩放预览和状态展示模板。

### 里程碑 B：首批可交互组件（已完成）

1. Button、IconButton、ToolButton、Toolbar。
2. Checkbox、Switch、TextField、Select、SegmentedControl。
3. FormRow、SettingsSection、Tooltip、Progress、Spinner、EmptyState、StatusBar。

### 里程碑 C：真实产品验证（Gallery 场景已完成，外部产品接入待实现）

1. 使用首个真实设置页验证表单、错误、长文本和焦点顺序。
2. 使用首个真实工具栏验证密度、快捷键、Tooltip 和选中态。
3. 修复公共组件后发布固定版本，业务项目不复制组件源码。

### 里程碑 D：常用桌面能力

按真实需求加入 PopupMenu、ContextMenu、Tabs、Table、Tree、ModalDialog、Toast、SplitPane、Form 和平台增强能力。

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
10. 对全局设计规范的引用，以及任何经过评审的例外。

组件规格不得重复定义 [`foundations.md`](foundations.md)、[`interaction.md`](interaction.md)、[`accessibility.md`](accessibility.md) 和 [`content-and-localization.md`](content-and-localization.md) 已经规定的通用行为。需要例外时必须说明原因、影响范围和替代验收方式。

## 13. 组件完成定义

组件同时满足以下条件才可标记为“已完成”：

- 规格文档和稳定公开 API 已评审。
- 默认、Hover、Pressed、Focused、Selected、Disabled、Loading 和错误状态按适用范围实现。
- 键盘操作、焦点顺序和可访问名称可用。
- Gallery 覆盖组件适用的变体、尺寸和状态，并提供密度、主题、Locale、RTL、长文本和 100%–200% 缩放环境；固定截图保留代表性组合。
- 状态组合、浮层层级、关闭策略和焦点恢复符合全局交互协议。
- 文本、图标、边框、焦点和功能色通过对比度与无障碍检查。
- 通过编译、交互测试、UTF-8 检查和当前 Windows 软件渲染截图人工评审；平台真机验收单独记录，不以语义声明代替。
- 没有业务概念、散落色值或平台 API 泄漏到核心层。

## 14. 官方参考

- [Ant Design 组件总览](https://ant.design/components/overview/)
- [Ant Design 主题与 Design Token](https://ant.design/docs/react/customize-theme/)
- [Ant Design 数据录入设计原则](https://ant.design/docs/spec/data-entry/)
- [Ant Design 数据展示设计原则](https://ant.design/docs/spec/data-display/)
- [Ant Design 按钮设计原则](https://ant.design/docs/spec/buttons/)
- [WCAG 2.2](https://www.w3.org/TR/WCAG22/)
- [W3C APG 键盘接口](https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/)

组件名称只用于建立共同语义。实现前仍须核对 Slint 标准组件与平台行为，不能假设 React 组件 API 可以直接移植。
